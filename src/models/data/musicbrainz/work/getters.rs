use color_eyre::eyre::{eyre, Context, OptionExt};
use itertools::Itertools;

use crate::core::entity_traits::has_id::HasID;
use crate::core::entity_traits::mbid::IsMbid;
use crate::models::data::entity_database::ENTITY_DATABASE;

use super::mbid::WorkMBID;
use super::Work;

impl Work {
    pub async fn get_parent_works_ids(&self) -> color_eyre::Result<Vec<WorkMBID>> {
        Ok(match &self.relations {
            Some(releases) => releases.clone(),
            None => {
                ENTITY_DATABASE.works().fetch_and_save(self.get_id().to_string())
                    .await
                    .context("Couldn't fetch data from the API")?
                    .ok_or_eyre(eyre!("Couldn't find any recording with the MBID"))?
                    .relations
                    .ok_or_eyre(eyre!(format!("Work is [`None`] after fetching from the API. Something wrong happened, as it should return a empty vec. \n Is there an include missing somewhere in the API call? Or is the credit not saved? Faulty requested recording ID is: {}", &self.id)))?
            }
        }.into_iter().filter(|relation| relation.content().is_work() && relation.direction() == "backward").map(|relation| relation.content().clone().unwrap_work()).collect_vec())
    }

    pub async fn get_all_parent_works_ids(&self) -> color_eyre::Result<Vec<WorkMBID>> {
        let work_ids = self.get_parent_works_ids().await?;

        let mut result_ids = Vec::new();
        for parent in work_ids {
            let new_parents = Box::pin(
                parent
                    .get_or_fetch_entity()
                    .await?
                    .get_all_parent_works_ids(),
            )
            .await?;
            result_ids.extend(new_parents.into_iter().collect_vec());
            result_ids.push(parent);
        }

        Ok(result_ids)
    }
}
