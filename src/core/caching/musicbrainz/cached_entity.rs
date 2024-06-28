use crate::core::caching::serde_cacache;
use crate::core::caching::serde_cacache::tidy::SerdeCacacheTidy;
use crate::models::data::musicbrainz::entity::is_musicbrainz_entity::IsMusicbrainzEntity;
use crate::models::data::musicbrainz::mbid::generic_mbid::NaiveMBID;
use crate::models::data::musicbrainz::mbid::is_musicbrainz_id::IsMusicbrainzID;
use crate::models::data::musicbrainz::musicbrainz_entity::MusicBrainzEntity;
use crate::models::data::musicbrainz::relation::external::RelationContentExt;
use crate::models::error::Error;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::sync::RwLockWriteGuard;

#[derive(Debug)]
pub struct CachedEntity<V>
where
    V: IsMusicbrainzEntity,
    MusicBrainzEntity: Into<Result<V, Error>>,
    NaiveMBID<V>: IsMusicbrainzID<V>,
{
    key: NaiveMBID<V>,
    loaded: RwLock<Option<Arc<V>>>,

    disk_cache: Arc<SerdeCacacheTidy<NaiveMBID<V>, V>>,
    alias_cache: Arc<SerdeCacacheTidy<NaiveMBID<V>, NaiveMBID<V>>>,
}

impl<V> CachedEntity<V>
where
    V: IsMusicbrainzEntity,
    MusicBrainzEntity: Into<Result<V, Error>>,
    NaiveMBID<V>: IsMusicbrainzID<V>,
{
    pub fn new(
        id: NaiveMBID<V>,
        disk_cache: Arc<SerdeCacacheTidy<NaiveMBID<V>, V>>,
        alias_cache: Arc<SerdeCacacheTidy<NaiveMBID<V>, NaiveMBID<V>>>,
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
        write_lock: &mut RwLockWriteGuard<'a, Option<Arc<V>>>,
    ) -> color_eyre::Result<Option<Arc<V>>> {
        if let Some(val) = write_lock.as_ref() {
            return Ok(Some(val.clone()));
        }

        self.inner_load(write_lock).await
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
        let maybe_value: Result<V, Error> = converted_fetch.0.into();
        let main_entity: Arc<V> = Arc::new(maybe_value?);

        self.alias_cache
            .set(&self.key, &main_entity.get_mbidspe().into_naive())
            .await?;
        self.update_with_lock(main_entity.clone(), write_lock)
            .await?;

        // Then, process the others
        // TODO: Use Stream
        for extra_entity in converted_fetch.1 {
            extra_entity.save_to_cache().await?;
        }

        Ok(main_entity)
    }

    // --- Insert ---

    /// Set a value in the value cache, its id in the alias cache and fill self
    ///
    /// This automatically picks a write lock
    pub async fn set(&self, value: Arc<V>) -> Result<(), serde_cacache::Error> {
        let mbid = value.get_mbidspe().into_naive();

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
        let mbid = value.get_mbidspe().into_naive();

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
            Some(older) => Arc::new(
                older
                    .as_ref()
                    .clone()
                    .partial_update(value.as_ref().clone()),
            ),
            None => value,
        };

        Ok(self.set(new_data).await?)
    }

    async fn update_with_lock<'a>(
        &self,
        value: Arc<V>,
        write_lock: &mut RwLockWriteGuard<'a, Option<Arc<V>>>,
    ) -> color_eyre::Result<()> {
        let older_version = self.get_or_load_with_lock(write_lock).await?;

        let new_data = match older_version {
            Some(older) => Arc::new(
                older
                    .as_ref()
                    .clone()
                    .partial_update(value.as_ref().clone()),
            ),
            None => value,
        };

        Ok(self.set_with_lock(new_data, write_lock).await?)
    }

    pub async fn update_from_generic_entity(
        &self,
        value: MusicBrainzEntity,
    ) -> color_eyre::Result<()> {
        let maybe_value: Result<V, Error> = value.into();
        let converted: Arc<V> = Arc::new(maybe_value?);
        self.update(converted).await
    }
}
