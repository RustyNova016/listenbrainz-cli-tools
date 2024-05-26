use crate::core::entity_traits::mb_cached::MBCached;
use color_eyre::eyre::{eyre, Context, OptionExt};
use itertools::Itertools;

use crate::core::entity_traits::mbid::IsMbid;

use super::mbid::WorkMBID;
use super::Work;

impl Work {
    pub async fn get_parent_works_ids(&self) -> color_eyre::Result<Vec<WorkMBID>> {
        Ok(match &self.relations {
            Some(releases) => releases.clone(),
            None => {
                self.refresh()
                    .await
                    .context("Couldn't fetch data from the API")?
                    .relations
                    .ok_or_eyre(eyre!(format!("Work is [`None`] after fetching from the API. Something wrong happened, as it should return a empty vec. \n Is there an include missing somewhere in the API call? Or is the credit not saved? Faulty requested work ID is: {}", &self.id)))?
            }
        }.into_iter().filter(|relation| relation.content().is_work() && relation.is_target_parent()).map(|relation| relation.content().clone().unwrap_work()).collect_vec())
    }

    pub async fn get_all_parent_works_ids(&self) -> color_eyre::Result<Vec<WorkMBID>> {
        let mut start_count = 0;
        let mut all_work_ids = self.get_parent_works_ids().await?;

        while start_count != all_work_ids.len() {
            start_count = all_work_ids.len();
            let mut iter_ids = Vec::new();

            for work_id in all_work_ids {
                iter_ids.extend(
                    work_id
                        .get_or_fetch_entity()
                        .await?
                        .get_parent_works_ids()
                        .await?,
                );
                iter_ids.push(work_id);
            }

            all_work_ids = iter_ids.into_iter().unique().collect_vec();
        }

        Ok(all_work_ids)
    }
}
