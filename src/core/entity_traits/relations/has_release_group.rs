use color_eyre::eyre::{eyre, Context, OptionExt};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::core::entity_traits::has_id::HasID;
use crate::core::entity_traits::mb_cached::MBCached;
use crate::core::entity_traits::mbid::IsMbid;
use crate::models::data::musicbrainz::release_group::mbid::ReleaseGroupMBID;

pub trait HasReleaseGroup<K: IsMbid<Self> + Serialize + DeserializeOwned>:
    HasID + MBCached<K>
{
    fn get_release_group(&self) -> &Option<ReleaseGroupMBID>;

    async fn get_or_fetch_release_group(&self) -> color_eyre::Result<ReleaseGroupMBID> {
        Ok(match &self.get_release_group() {
            Some(credits) => credits.clone(),
            None => {
                self.refresh()
                    .await
                    .context("Couldn't fetch data from the API")?
                    .get_release_group()
                    .as_ref()
                    .ok_or_eyre(eyre!(format!("Artist credit is [`None`] after fetching from the API. Something wrong happened, as it should return a empty vec. \n Is there an include missing somewhere in the API call? Or is the credit not saved? Faulty requested entity ID is: {}", &self.get_id())))?
                    .clone()
            }
        })
    }
}

pub trait HasReleaseGroups<K: IsMbid<Self> + Serialize + DeserializeOwned>:
    HasID + MBCached<K>
{
    fn get_release_groups(&self) -> &Option<Vec<ReleaseGroupMBID>>;

    async fn get_or_fetch_release_groups(&self) -> color_eyre::Result<Vec<ReleaseGroupMBID>> {
        Ok(match &self.get_release_groups() {
            Some(credits) => credits.clone(),
            None => {
                self.refresh()
                    .await
                    .context("Couldn't fetch data from the API")?
                    .get_release_groups()
                    .as_ref()
                    .ok_or_eyre(eyre!(format!("Artist credit is [`None`] after fetching from the API. Something wrong happened, as it should return a empty vec. \n Is there an include missing somewhere in the API call? Or is the credit not saved? Faulty requested entity ID is: {}", &self.get_id())))?
                    .clone()
            }
        })
    }
}
