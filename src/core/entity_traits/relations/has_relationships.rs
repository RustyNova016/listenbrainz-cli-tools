use color_eyre::eyre::{eyre, Context, OptionExt};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::core::entity_traits::has_id::HasID;
use crate::core::entity_traits::mb_cached::MBCached;
use crate::core::entity_traits::mbid::IsMbid;
use crate::models::data::musicbrainz::relation::Relation;

pub trait HasRelationShips<K: IsMbid<Self> + Serialize + DeserializeOwned>:
    HasID + MBCached<K>
{
    fn get_relationships(&self) -> &Option<Vec<Relation>>;

    async fn get_or_fetch_relationships(&self) -> color_eyre::Result<Vec<Relation>> {
        Ok(match &self.get_relationships() {
            Some(credits) => credits.clone(),
            None => {
                self.refresh()
                    .await
                    .context("Couldn't fetch data from the API")?
                    .get_relationships()
                    .as_ref()
                    .ok_or_eyre(eyre!(format!("Artist credits field is [`None`] after fetching from the API. Something wrong happened, as it should return a empty vec. \n Is there an include missing somewhere in the API call? Or is the credit not saved? Faulty requested entity ID is: {}", &self.get_id())))?
                    .clone()
            }
        })
    }
}
