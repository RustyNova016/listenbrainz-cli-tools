use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;

use chashmap::CHashMap;
use color_eyre::eyre::Context;
use color_eyre::owo_colors::OwoColorize;
use futures::try_join;
use serde::de::DeserializeOwned;
use serde::Serialize;
use tokio::sync::{OnceCell, RwLock, RwLockWriteGuard, Semaphore};

use crate::core::caching::serde_cacache::error::Error;
use crate::core::caching::serde_cacache::tidy::SerdeCacacheTidy;
use crate::core::caching::CACHE_LOCATION;
use crate::core::entity_traits::mbid::{HasMBID, IsMbid};
use crate::core::entity_traits::updatable::Updatable;
use crate::models::data::musicbrainz::external_musicbrainz_entity::FlattenedMBEntityExt;
use crate::models::data::musicbrainz::relation::external::RelationContentExt;
use crate::utils::{println_cli, println_cli_warn};

#[derive(Debug)]
pub struct MusicbrainzCache<K, V>
where
    K: IsMbid<V> + Serialize + DeserializeOwned,
    V: Serialize + DeserializeOwned + HasMBID<K> + Updatable + Clone,
{
    ram_cache: RwLock<HashMap<String, Arc<OnceCell<Arc<V>>>>>,
    disk_cache: SerdeCacacheTidy<K, V>,
    alias_cache: SerdeCacacheTidy<K, K>,

    value_locks: Arc<CHashMap<String, Arc<RwLock<String>>>>,

    // Keep the locks of the fetch operations.
    fetch_locks: CHashMap<String, Arc<Semaphore>>,
}

impl<K: IsMbid<V>, V> MusicbrainzCache<K, V>
where
    K: IsMbid<V> + Serialize + DeserializeOwned,
    V: Serialize + DeserializeOwned + HasMBID<K> + Updatable + Clone,
{
    pub fn new(name: &str) -> Self {
        let mut location = CACHE_LOCATION.clone();
        location.push(name);

        let mut alias_location = CACHE_LOCATION.clone();
        alias_location.push(format!("{name}_aliases"));

        Self {
            ram_cache: RwLock::new(HashMap::new()),
            disk_cache: SerdeCacacheTidy::new(location),
            alias_cache: SerdeCacacheTidy::new(alias_location),
            value_locks: Arc::new(CHashMap::new()),
            fetch_locks: CHashMap::new(),
        }
    }

    pub async fn get_from_ram(&self, mbid: &K) -> Option<Arc<V>> {
        self.ram_cache
            .read()
            .await
            .get(&mbid.to_string())
            .and_then(|cell| cell.get().cloned())
    }

    pub async fn get_from_disk(&self, mbid: &K) -> Result<Option<Arc<V>>, Error> {
        self.disk_cache
            .get_or_option(mbid)
            .await
            .map(|opt| opt.map(|val| Arc::new(val)))
    }

    pub async fn get_or_fetched(&self, mbid: &K) -> color_eyre::Result<Arc<V>> {
        let ram_cell = self.get_ram_cell(mbid).await;
        let disk_cache = self.disk_cache.clone();

        if let Some(data) = ram_cell.get() {
            return Ok(data.clone());
        }

        ram_cell
            .get_or_try_init(|| async move {
                // First, try init from disk
                let disk_data = disk_cache
                    .get_or_option(mbid)
                    .await
                    .map(|opt| opt.map(|val| Arc::new(val)))?;

                if let Some(data) = disk_data {
                    return Ok(data);
                }

                // No data? Fetch it
                Self::fetch_and_save(mbid).await?;

                Ok(disk_cache
                    .get_or_option(mbid)
                    .await
                    .map(|opt| opt.map(|val| Arc::new(val)))?
                    .expect("Fetched data should be in the cache"))
            })
            .await
            .cloned()
    }

    pub async fn get_ram_cell(&self, mbid: &K) -> Arc<OnceCell<Arc<V>>> {
        let mut ram_disk = self.ram_cache.write().await;

        if let Some(cell) = ram_disk.get(&mbid.to_string()) {
            return cell.clone();
        }

        ram_disk.insert(mbid.to_string(), Arc::new(OnceCell::new()));
        ram_disk.get(&mbid.to_string()).unwrap().clone()
    }

    pub async fn force_fetch_entity(&self, mbid: &K) -> color_eyre::Result<Arc<V>> {
        self.remove(mbid).await?;
        self.get_or_fetched(mbid).await
    }

    pub async fn get(&self, mbid: &K) -> Result<Option<V>, Error> {
        let new_mbid = self.get_primary_mbid_alias(mbid).await?;

        //if new_mbid.to_string() != mbid.to_string() {
        //    println_cli(format!("    Aliasing {mbid} -> {new_mbid}"))
        //}

        let mbid = new_mbid;

        let lock = self.get_lock(&mbid);
        let _read_lock = lock.read().await;

        match self.disk_cache.get_or_option(&mbid).await {
            // Cache hit
            Ok(Some(val)) => {
                //println_cli(format!("Cache hit for mbid {mbid}"));
                Ok(Some(val))
            }

            // Cache miss
            Ok(None) => {
                //println_cli(format!("Cache miss for mbid {mbid}"));
                Ok(None)
            }

            // Something went wrong while deserializing the struct
            // Schema probably changed. Which means we need make the cache hit fail
            Err(Error::CacheDeserializationError(_err)) => {
                //println_cli(
                //    format!("Cache hit but with deserialization error for mbid {mbid}").yellow(),
                //);
                println_cli(format!("Couldn't retrieve cache data for mbid {mbid}").yellow());
                //println_cli(err);
                Ok(None)
            }

            Err(val) => Err(val),
        }
    }

    pub async fn set(&self, value: &V) -> Result<(), Error> {
        let mbid = value.get_mbid();

        let lock = self.get_lock(&mbid);
        let _write_lock = lock.write().await;

        // TODO: Add tokio::join! for speedup.
        self.alias_cache.set(&mbid, &mbid).await?;
        self.disk_cache.set(&mbid, value).await?;
        Ok(())
    }

    pub async fn update(&self, value: &V) -> color_eyre::Result<()> {
        let mbid = value.get_mbid();
        let older = self.get(&mbid).await?;

        //println_cli(format!("Updating {mbid}"));

        if let Some(older) = older {
            self.set(&older.update(value.clone())).await?;
        } else {
            self.set(value).await?;
        }

        Ok(())
    }

    pub async fn invalidate_last_entries(
        &self,
        k: usize,
        keep_min: usize,
    ) -> color_eyre::Result<()> {
        self.disk_cache.delete_last_entries(k, keep_min).await?;
        Ok(())
    }

    pub async fn insert_alias(&self, alias: &K, main: &K) -> Result<(), Error> {
        self.alias_cache.set(alias, main).await?;
        Ok(())
    }

    pub async fn remove(&self, id: &K) -> color_eyre::Result<()> {
        self.alias_cache.remove(id).await?;
        self.ram_cache.write().await.remove(&id.to_string());
        self.disk_cache.remove(id).await?;
        Ok(())
    }

    pub async fn set_with_lock<'a>(
        &self,
        value: &V,
        _lock: RwLockWriteGuard<'a, String>,
    ) -> Result<(), Error> {
        let mbid = value.get_mbid();

        // TODO: Add tokio::join! for speedup.
        self.alias_cache.set(&mbid, &mbid).await?;
        self.disk_cache.set(&mbid, value).await?;
        Ok(())
    }

    pub async fn get_primary_mbid_alias(&self, mbid: &K) -> Result<K, Error> {
        match self.alias_cache.get_or_option(mbid).await {
            Ok(Some(val)) => Ok(val),
            Ok(None) | Err(Error::CacheDeserializationError(_)) => {
                #[cfg(debug_assertions)]
                println_cli_warn("Trying to fetch the primary alias of MBID resulted in `None`. Returning input instead");
                Ok(mbid.clone())
            }
            Err(val) => Err(val),
        }
    }

    pub async fn get_or_fetch_primary_mbid_alias(&self, mbid: &K) -> color_eyre::Result<K> {
        match self.alias_cache.get_or_option(mbid).await {
            Ok(Some(val)) => Ok(val),
            Ok(None) | Err(Error::CacheDeserializationError(_)) => {
                Ok(self.force_fetch_entity(mbid).await?.get_mbid())
            }
            Err(val) => Err(val.into()),
        }
    }

    #[must_use]
    fn get_lock(&self, key: &K) -> Arc<RwLock<String>> {
        let key = key.to_string();
        match self.value_locks.get(&key) {
            Some(val) => val.deref().clone(),

            None => {
                self.value_locks
                    .insert(key.to_string(), Arc::new(RwLock::new(key.clone())));

                self.value_locks
                    .get(&key)
                    .expect("Couldn't get just inserted lock")
                    .deref()
                    .clone()
            }
        }
    }

    #[must_use]
    fn get_fetch_lock(&self, key: &K) -> Arc<Semaphore> {
        if let Some(semaphore) = self.fetch_locks.get(&key.to_string()) {
            return (*semaphore).clone();
        }

        self.fetch_locks
            .insert(key.to_string(), Arc::new(Semaphore::new(1)));
        return (*self
            .fetch_locks
            .get(&key.to_string())
            .expect("Couldn't get a new semaphore"))
        .clone();
    }

    #[deprecated]
    pub async fn get_or_fetch(&self, mbid: &K) -> color_eyre::Result<V> {
        // Let's try getting the value:
        if let Ok(Some(result)) = self.get(mbid).await {
            return Ok(result);
        }

        // So no cache hit? Alright. We start a fetch operation.
        // Let's get a fetching permit first, to signal others to wait until we get the result
        let lock = self.get_fetch_lock(mbid);
        let _permit = lock.acquire().await.context("Couldn't get permit")?;

        // Now we recheck the cache. While getting the permit, there might have already been an operation that populated it.
        if let Ok(Some(result)) = self.get(mbid).await {
            return Ok(result);
        }

        // So now, we are sure the cache is empty, and that we're the only one doing this operation application wide.
        // Then it's time to fetch!

        Self::fetch_and_save(mbid).await?;

        Ok(self
            .get(mbid)
            .await?
            .expect("Fetched data should be in the cache"))
    }

    async fn fetch_and_save(mbid: &K) -> color_eyre::Result<()> {
        let fetch_result = mbid.fetch().await?;
        let converted_fetch = fetch_result.flattened();

        converted_fetch
            .insert_into_cache_with_alias(&mbid.clone().into_mbid())
            .await
    }

    /// Fetch an item, bypassing the cache. This also save the request.
    /// Only one request is allowed at a time, so a Semaphore permit is required.
    /// If none is provided, it will get assigned automatically.
    ///
    /// ⚠️ Waiting for a permit doesn't cancel the request. It only delays it.
    /// If the intention is to only fetch once, see [`Self::get_or_fetch`]
    #[deprecated]
    pub async fn force_fetch_and_save(&self, mbid: &K) -> color_eyre::Result<V> {
        let lock = self.get_fetch_lock(mbid);
        let _permit = lock.acquire().await.context("Couldn't get permit")?;

        //println_cli(format!("Pre refresh: {:#?}", self.get(mbid).await?));

        Self::fetch_and_save(mbid).await?;

        //println_cli(format!("Post refresh: {:#?}", self.get(mbid).await?));

        Ok(self
            .get(mbid)
            .await?
            .expect("Fetched data should be in the cache"))
    }

    pub async fn clear(&self) -> cacache::Result<()> {
        let _ = try_join!(self.alias_cache.clear(), self.disk_cache.clear())?;

        Ok(())
    }
}
