use std::ops::Deref;
use std::sync::Arc;

use chashmap::CHashMap;
use color_eyre::eyre::Context;
use serde::de::DeserializeOwned;
use serde::Serialize;
use tokio::sync::{RwLock, RwLockWriteGuard, Semaphore};

use crate::core::caching::CACHE_LOCATION;
use crate::core::caching::serde_cacache::error::Error;
use crate::core::caching::serde_cacache::tidy::SerdeCacacheTidy;
use crate::core::entity_traits::mbid::{HasMBID, IsMbid};
use crate::core::entity_traits::updatable::Updatable;
use crate::models::data::musicbrainz::external_musicbrainz_entity::ExternalMusicBrainzEntityExt;
use crate::models::data::musicbrainz_database::MUSICBRAINZ_DATABASE;

#[derive(Debug)]
pub struct MusicbrainzCache<K, V>
    where
        K: IsMbid<V> + Serialize + DeserializeOwned,
        V: Serialize + DeserializeOwned + HasMBID<K>  + Updatable + Clone,
{
    disk_cache: SerdeCacacheTidy<K, V>,
    alias_cache: SerdeCacacheTidy<K, K>,

    value_locks: Arc<CHashMap<String, Arc<RwLock<String>>>>,

    // Keep the locks of the fetch operations.
    fetch_locks: CHashMap<String, Arc<Semaphore>>,
}

impl<K: IsMbid<V>, V> MusicbrainzCache<K, V>
where
    K: IsMbid<V> + Serialize + DeserializeOwned,
    V: Serialize + DeserializeOwned + HasMBID<K>  + Updatable + Clone,
{
    pub fn new(name: &str) -> Self {
        let mut location = CACHE_LOCATION.clone();
        location.push(name);

        let mut alias_location = CACHE_LOCATION.clone();
        alias_location.push(format!("{name}_aliases"));

        Self {
            disk_cache: SerdeCacacheTidy::new(location),
            alias_cache: SerdeCacacheTidy::new(alias_location),
            value_locks: Arc::new(CHashMap::new()),
            fetch_locks: CHashMap::new(),
        }
    }

    pub async fn get(&self, mbid: &K) -> Result<Option<V>, Error> {
        let mbid = self.get_primary_mbid_alias(mbid).await?;

        let lock = self.get_lock(&mbid);
        let _read_lock = lock.read().await;

        match self.disk_cache.get_or_option(&mbid).await {
            Ok(val) => Ok(val),
            Err(Error::CacheDeserializationError(_)) => Ok(None), // Schema probably changed. Which means we need make the cache hit fail
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

        if let Some(older) = older {
            self.set(&older.update(value.clone())).await?;
        } else {
            self.set(value).await?;
        }

        Ok(())
    }

    pub async fn invalidate_last_entries(&self, k: usize, keep_min: usize) -> color_eyre::Result<()> {
        self.disk_cache.delete_last_entries(k, keep_min).await?;
        Ok(())
    }

    pub async fn insert_alias(&self, alias: &K, main: &K) -> Result<(), Error> {
        self.alias_cache.set(alias, main).await?;
        Ok(())
    }

    pub async fn remove(&self, id: &K) -> color_eyre::Result<()> {
        self.alias_cache.remove(id).await?;
        self.disk_cache.remove(id).await?;
        Ok(())
    }

    pub async fn set_with_lock<'a>(&self, value: &V, _lock: RwLockWriteGuard<'a, String>) -> Result<(), Error> {
        let mbid = value.get_mbid();

        // TODO: Add tokio::join! for speedup.
        self.alias_cache.set(&mbid, &mbid).await?;
        self.disk_cache.set(&mbid, value).await?;
        Ok(())
    }

    pub async fn get_primary_mbid_alias(&self, mbid: &K) -> Result<K, Error> {
        match self.alias_cache.get_or_option(mbid).await {
            Ok(Some(val)) => Ok(val),
            Ok(None) | Err(Error::CacheDeserializationError(_)) => Ok(mbid.clone()),
            Err(val) => Err(val),
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
        let fetch_result = mbid.fetch().await?;
        let converted_fetch = fetch_result.flattened();

        // Let's take care of the main data
        converted_fetch.0.save_to_cache().await?;
        MUSICBRAINZ_DATABASE.add_alias(&mbid.clone().into_mbid(), &converted_fetch.0.get_mbid()).await?;

        for extra in converted_fetch.1 {
            extra.save_to_cache().await?;
        }

        Ok(self.get(mbid).await?.expect("Fetched data should be in the cache"))
    }
}
