use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;

use futures::try_join;
use serde::de::DeserializeOwned;
use serde::Serialize;
use tokio::sync::{RwLock, RwLockWriteGuard};

use crate::core::caching::serde_cacache::error::Error;
use crate::core::caching::serde_cacache::tidy::SerdeCacacheTidy;
use crate::core::caching::CACHE_LOCATION;
use crate::core::entity_traits::mbid::{HasMBID, IsMbid};
use crate::core::entity_traits::updatable::Updatable;
use crate::models::data::musicbrainz::musicbrainz_entity::AnyMusicBrainzEntity;
use crate::models::data::musicbrainz::relation::external::RelationContentExt;
use crate::utils::println_cli_warn;

use super::serde_cacache;

#[derive(Debug)]
pub struct MusicbrainzCache<K, V>
where
    K: IsMbid<V> + Serialize + DeserializeOwned,
    V: Serialize + DeserializeOwned + HasMBID<K> + Updatable + Clone,
{
    cache_entities: RwLock<HashMap<K, Arc<CachedEntity<K, V>>>>,

    disk_cache: Arc<SerdeCacacheTidy<K, V>>,
    alias_cache: Arc<SerdeCacacheTidy<K, K>>,
}

impl<K: IsMbid<V>, V> MusicbrainzCache<K, V>
where
    K: IsMbid<V> + Serialize + DeserializeOwned + Eq + Hash,
    V: Serialize + DeserializeOwned + HasMBID<K> + Updatable + Clone,
{
    pub fn new(name: &str) -> Self {
        let mut location = CACHE_LOCATION.clone();
        location.push(name);

        let mut alias_location = CACHE_LOCATION.clone();
        alias_location.push(format!("{name}_aliases"));

        Self {
            cache_entities: RwLock::new(HashMap::new()),
            disk_cache: Arc::new(SerdeCacacheTidy::new(location)),
            alias_cache: Arc::new(SerdeCacacheTidy::new(alias_location)),
        }
    }

    pub async fn get_entity(&self, id: &K) -> Arc<CachedEntity<K, V>> {
        // Use a read to get the entity
        if let Some(entity) = self.cache_entities.read().await.get(id) {
            return entity.clone();
        }

        // The entity isn't found. Let's get into exclusive write mode
        let mut map = self.cache_entities.write().await;

        // While we waited for a write, it may have initialized the entity. Let's recheck
        if let Some(entity) = map.get(id) {
            return entity.clone();
        }

        // No entity was found so we initialize it
        let entity = Arc::new(CachedEntity::new(
            id.clone(),
            self.disk_cache.clone(),
            self.alias_cache.clone(),
        ));
        map.insert(id.clone(), entity.clone());
        entity
    }

    pub async fn get_load_or_fetch(&self, mbid: &K) -> color_eyre::Result<Arc<V>> {
        self.get_entity(mbid).await.get_load_or_fetch().await
    }

    pub async fn force_fetch_entity(&self, mbid: &K) -> color_eyre::Result<Arc<V>> {
        self.remove(mbid).await?;
        self.get_load_or_fetch(mbid).await
    }

    pub async fn set(&self, value: Arc<V>) -> Result<(), Error> {
        self.get_entity(&value.get_mbid()).await.set(value).await
    }

    pub async fn update(&self, value: Arc<V>) -> color_eyre::Result<()> {
        self.get_entity(&value.get_mbid()).await.update(value).await
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
        self.cache_entities.write().await.remove(id);
        self.alias_cache.remove(id).await?;
        self.disk_cache.remove(id).await?;
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

    pub async fn clear(&self) -> cacache::Result<()> {
        let _ = try_join!(self.alias_cache.clear(), self.disk_cache.clear())?;
        self.cache_entities.write().await.clear();

        Ok(())
    }
}

#[derive(Debug)]
pub struct CachedEntity<K, V>
where
    K: IsMbid<V> + Serialize + DeserializeOwned,
    V: Serialize + DeserializeOwned + HasMBID<K> + Updatable + Clone,
{
    key: K,
    loaded: RwLock<Option<Arc<V>>>,

    disk_cache: Arc<SerdeCacacheTidy<K, V>>,
    alias_cache: Arc<SerdeCacacheTidy<K, K>>,
}

impl<K, V> CachedEntity<K, V>
where
    K: IsMbid<V> + Serialize + DeserializeOwned,
    V: Serialize + DeserializeOwned + HasMBID<K> + Updatable + Clone,
{
    pub fn new(
        id: K,
        disk_cache: Arc<SerdeCacacheTidy<K, V>>,
        alias_cache: Arc<SerdeCacacheTidy<K, K>>,
    ) -> Self {
        Self {
            alias_cache,
            disk_cache,
            key: id,
            loaded: RwLock::new(None),
        }
    }

    /// **Get** from the loaded value, or **load** from the cache.
    ///
    /// This version create its own read lock in case of a **get**, and create a write lock in case of **load**.
    pub async fn get_or_load(&self) -> color_eyre::Result<Option<Arc<V>>> {
        let get_result = self.get_or_lock().await;

        match get_result {
            Ok(val) => Ok(Some(val)),
            Err(mut write_lock) => self.inner_load(&mut write_lock).await,
        }
    }

    /// **Get** from the loaded value, or **load** from the cache.
    ///
    /// This version take an external write lock
    pub async fn get_or_load_with_lock<'a>(
        &self,
        mut write_lock: &mut RwLockWriteGuard<'a, Option<Arc<V>>>,
    ) -> color_eyre::Result<Option<Arc<V>>> {
        if let Some(val) = write_lock.as_ref() {
            return Ok(Some(val.clone()));
        }

        self.inner_load(&mut write_lock).await
    }

    /// **Get** from the loaded value, or **load** from the cache, or **fetch** from the MB database
    pub async fn get_load_or_fetch(&self) -> color_eyre::Result<Arc<V>> {
        let get_result = self.get_or_lock().await;

        match get_result {
            Ok(val) => Ok(val),
            Err(mut write_lock) => {
                if let Some(val) = self.inner_load(&mut write_lock).await? {
                    return Ok(val.clone());
                }

                self.inner_fetch(&mut write_lock).await
            }
        }
    }

    pub async fn get(&self) -> Option<Arc<V>> {
        self.loaded.read().await.clone()
    }

    /// Tries to get the value, but if none, get a write lock.
    /// If a write lock is already held, it will recheck if the entity was loaded upon obtaining it.
    #[allow(clippy::needless_lifetimes)]
    pub async fn get_or_lock<'a>(&'a self) -> Result<Arc<V>, RwLockWriteGuard<'a, Option<Arc<V>>>> {
        if let Some(val) = self.get().await {
            return Ok(val);
        }

        let write_lock = self.loaded.write().await;
        if let Some(val) = write_lock.as_ref() {
            return Ok(val.clone());
        }

        Err(write_lock)
    }

    async fn inner_load<'a>(
        &self,
        write_lock: &mut RwLockWriteGuard<'a, Option<Arc<V>>>,
    ) -> color_eyre::Result<Option<Arc<V>>> {
        let cached = self
            .disk_cache
            .get_or_option(&self.key)
            .await?
            .map(|val| Arc::new(val));

        if let Some(val) = cached.clone() {
            write_lock.replace(val);
        }

        Ok(cached)
    }

    async fn inner_fetch<'a>(
        &self,
        write_lock: &mut RwLockWriteGuard<'a, Option<Arc<V>>>,
    ) -> color_eyre::Result<Arc<V>> {
        let fetch_result = self.key.fetch().await?;
        let converted_fetch = fetch_result.flattened();

        // First, process the main entity
        let main_entity: Arc<V> = Arc::new(converted_fetch.0.into());
        self.alias_cache.set(&self.key, &main_entity.get_mbid());
        self.update_with_lock(main_entity.clone(), write_lock).await;

        // Then, process the others
        for extra_entity in converted_fetch.1 {
            extra_entity.update_cache().await?;
        }

        Ok(main_entity)
    }

    // --- Insert ---

    /// Set a value in the value cache, its id in the alias cache and fill self
    ///
    /// This automatically picks a write lock
    pub async fn set(&self, value: Arc<V>) -> Result<(), serde_cacache::Error> {
        let mbid = value.get_mbid();

        // TODO: Add try_join! for speedup.
        self.loaded.write().await.replace(value.clone());
        self.alias_cache.set(&mbid, &mbid).await?;
        self.disk_cache.set(&mbid, value.as_ref()).await?;
        Ok(())
    }

    /// Set a value in the value cache, its id in the alias cache and fill self
    ///
    /// This version requiert a write lock
    pub async fn set_with_lock<'a>(
        &self,
        value: Arc<V>,
        write_lock: &mut RwLockWriteGuard<'a, Option<Arc<V>>>,
    ) -> Result<(), serde_cacache::Error> {
        let mbid = value.get_mbid();

        // TODO: Add try_join! for speedup.
        write_lock.replace(value.clone());
        self.alias_cache.set(&mbid, &mbid).await?;
        self.disk_cache.set(&mbid, value.as_ref()).await?;
        Ok(())
    }

    // --- Update ---

    pub async fn update(&self, value: Arc<V>) -> color_eyre::Result<()> {
        let older_version = self.get_or_load().await?;

        let new_data = match older_version {
            Some(older) => older.update(value.clone()),
            None => value,
        };

        Ok(self.set(new_data).await?)
    }

    async fn update_with_lock<'a>(
        &self,
        value: Arc<V>,
        mut write_lock: &mut RwLockWriteGuard<'a, Option<Arc<V>>>,
    ) -> color_eyre::Result<()> {
        let older_version = self.get_or_load_with_lock(write_lock).await?;

        let new_data = match older_version {
            Some(older) => older.update(value.clone()),
            None => value,
        };

        Ok(self.set_with_lock(new_data, &mut write_lock).await?)
    }

    pub async fn update_from_generic_entity(
        &self,
        value: AnyMusicBrainzEntity,
    ) -> color_eyre::Result<()> {
        let converted: Arc<V> = value.try_into()?;
        self.update(converted).await
    }
}
