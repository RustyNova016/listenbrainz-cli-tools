use crate::core::caching::serde_cacache;
use crate::models::data::musicbrainz::entity::traits::MusicBrainzEntity;
use crate::models::data::musicbrainz::mbid::state_id::state::PrimaryMBID;
use crate::models::error::Error;

use super::CachedEntity;

impl<V> CachedEntity<V>
where
    V: MusicBrainzEntity,
{
    /// Set the primary id for the entity.
    pub async fn set_primary_id(&mut self, mbid: PrimaryMBID<V>) -> color_eyre::Result<()> {
        self.alias_cache.set(&self.key, &mbid).await?;
        self.primary_id = Some(mbid);
        Ok(())
    }

    /// Return the primary id of the entity,
    pub async fn get_or_load_primary_id(&mut self) -> Result<Option<PrimaryMBID<V>>, serde_cacache::Error> {
        if let Some(primary_id) = &self.primary_id {
            return Ok(Some(primary_id.clone()))
        }

        let alias = self.alias_cache.get_or_option(&self.key).await?;
        self.primary_id = alias.clone();
        Ok(alias)
    }

    /// This function return the id of the value to load/fetch. 
    /// 
    /// If the key of the current entity isn't the same as the id to fetch, then it will return a error. 
    /// 
    /// If it is unknown whether the key is the correct one, it will return it
    pub async fn get_verified_id(&mut self) -> Result<PrimaryMBID<V>, Error> {
        let id = self.get_or_load_primary_id().await?.unwrap_or_else(|| PrimaryMBID::from(self.key.to_string()));

        if id.to_string() == self.key.to_string() {
            return Ok(id)
        }

        Err(Error::MBIDRedirectError(id.to_string()))
    }
}