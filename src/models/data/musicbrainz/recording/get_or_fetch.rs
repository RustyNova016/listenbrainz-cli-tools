use super::Recording;
use crate::core::entity_traits::mbid::MBID;
use crate::core::entity_traits::relations::has_artist_credits::HasArtistCredits;
use crate::models::data::entity_database::ENTITY_DATABASE;
use crate::models::data::musicbrainz::artist_credit::collection::ArtistCredits;
use crate::models::data::musicbrainz::release::mbid::ReleaseMBID;
use crate::models::data::musicbrainz::work::mbid::WorkMBID;
use crate::models::data::musicbrainz::HasMbid;
use color_eyre::eyre::{eyre, Context, OptionExt};
use itertools::Itertools;

impl Recording {
    pub async fn get_or_fetch_releases_ids(&self) -> color_eyre::Result<Vec<ReleaseMBID>> {
        Ok(match &self.releases {
            Some(releases) => releases.clone(),
            None => {
                ENTITY_DATABASE.recordings().fetch_and_save(self.get_mbid().to_string())
                    .await
                    .context("Couldn't fetch data from the API")?
                    .ok_or_eyre(eyre!("Couldn't find any recording with the MBID"))?
                    .releases
                    .ok_or_eyre(eyre!(format!("Releases is [`None`] after fetching from the API. Something wrong happened, as it should return a empty vec. \n Is there an include missing somewhere in the API call? Or is the credit not saved? Faulty requested recording ID is: {}", &self.id)))?
            }
        })
    }

    pub async fn get_or_fetch_work_ids(&self) -> color_eyre::Result<Vec<WorkMBID>> {
        Ok(match &self.relations {
            Some(releases) => releases.clone(),
            None => {
                ENTITY_DATABASE.recordings().fetch_and_save(self.get_mbid().to_string())
                    .await
                    .context("Couldn't fetch data from the API")?
                    .ok_or_eyre(eyre!("Couldn't find any recording with the MBID"))?
                    .relations
                    .ok_or_eyre(eyre!(format!("Work is [`None`] after fetching from the API. Something wrong happened, as it should return a empty vec. \n Is there an include missing somewhere in the API call? Or is the credit not saved? Faulty requested recording ID is: {}", &self.id)))?
            }
        }.into_iter().filter(|relation| relation.content().is_work()).map(|relation| relation.content().clone().unwrap_work()).collect_vec())
    }

    pub async fn get_or_fetch_work_ids_with_parents(&self) -> color_eyre::Result<Vec<WorkMBID>> {
        let work_ids = self.get_or_fetch_work_ids().await?;

        if work_ids.is_empty() {
            return Ok(work_ids);
        }

        let mut result_ids = Vec::new();
        for parent in work_ids {
            result_ids.extend(
                parent
                    .get_or_fetch_entity()
                    .await?
                    .get_all_parent_works_ids()
                    .await?,
            );
            result_ids.push(parent);
        }

        Ok(result_ids)
    }
}

impl HasArtistCredits for Recording {
    fn get_artist_credits(&self) -> &Option<ArtistCredits> {
        &self.artist_credit
    }
}
