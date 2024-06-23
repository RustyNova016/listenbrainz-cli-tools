use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::core::entity_traits::has_id::HasID;
use crate::core::entity_traits::mb_cached::MBCached;
use crate::core::entity_traits::mbid::IsMbid;
use crate::models::error::Error;

pub trait HasRelationshipGeneric<K: IsMbid<Self> + Serialize + DeserializeOwned, V>:
    HasID + MBCached<K>
{
    fn get_relations(&self) -> Option<V>;

    async fn get_or_fetch_relations(&self) -> color_eyre::Result<V> {
        // Try accessing directly from the data
        if let Some(data) = self.get_relations() {
            return Ok(data);
        }

        // Try accessing from the cache
        if let Some(data) = self.get_mbid().get_or_fetch_entity().await?.get_relations() {
            return Ok(data);
        }

        // Try refreshing then accessing it
        if let Some(data) = Self::get_cache()
            .force_fetch_entity(&self.get_mbid())
            .await?
            .get_relations()
        {
            return Ok(data);
        }

        // Alright there's a problem here. Let's throw an error
        Err(Error::NoneAfterFetch("generic".to_string(), self.get_mbid().to_string()).into())
    }
}
