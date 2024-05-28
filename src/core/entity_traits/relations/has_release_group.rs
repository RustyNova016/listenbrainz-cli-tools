use color_eyre::eyre::{eyre, Context, OptionExt};

use crate::core::entity_traits::cached::Cached;
use crate::core::entity_traits::fetchable::Fetchable;
use crate::core::entity_traits::has_id::HasID;
use crate::models::data::musicbrainz::release_group::mbid::ReleaseGroupMBID;

pub trait HasReleaseGroup: HasID + Cached + Fetchable {
    fn get_release_group(&self) -> &Option<ReleaseGroupMBID>;

    async fn get_or_fetch_release_group(&self) -> color_eyre::Result<ReleaseGroupMBID> {
        Ok(match &self.get_release_group() {
            Some(credits) => credits.clone(),
            None => {
                Self::get_cache().fetch_and_save(self.get_id().to_string())
                    .await
                    .context("Couldn't fetch data from the API")?
                    .ok_or_eyre(eyre!("Couldn't find any release with the MBID"))?
                    .get_release_group()
                    .as_ref()
                    .ok_or_eyre(eyre!(format!("Artist credit is [`None`] after fetching from the API. Something wrong happened, as it should return a empty vec. \n Is there an include missing somewhere in the API call? Or is the credit not saved? Faulty requested entity ID is: {}", &self.get_id())))?
                    .clone()
            }
        })
    }
}

pub trait HasReleaseGroups: HasID + Cached + Fetchable {
    fn get_release_groups(&self) -> &Option<Vec<ReleaseGroupMBID>>;

    async fn get_or_fetch_release_groups(&self) -> color_eyre::Result<Vec<ReleaseGroupMBID>> {
        Ok(match &self.get_release_groups() {
            Some(credits) => credits.clone(),
            None => {
                Self::get_cache().fetch_and_save(self.get_id().to_string())
                    .await
                    .context("Couldn't fetch data from the API")?
                    .ok_or_eyre(eyre!("Couldn't find any release with the MBID"))?
                    .get_release_groups()
                    .as_ref()
                    .ok_or_eyre(eyre!(format!("Artist credit is [`None`] after fetching from the API. Something wrong happened, as it should return a empty vec. \n Is there an include missing somewhere in the API call? Or is the credit not saved? Faulty requested entity ID is: {}", &self.get_id())))?
                    .clone()
            }
        })
    }
}
