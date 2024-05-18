use chrono::prelude::Utc;
use itertools::Itertools;

use crate::core::statistics::listen_rate::ListenRate;
use crate::models::cli::common::GroupByTarget;

use super::ListenCollection;

impl ListenCollection {
    pub async fn get_listen_rates(
        &self,
    ) -> color_eyre::Result<Vec<(ListenCollection, ListenRate)>> {
        let stats = self.get_statistics_of(GroupByTarget::Recording).await?;
        let now = Utc::now();

        Ok(stats
            .into_vec()
            .into_iter()
            .map(|(recording_id, listens)| {
                let duration = now
                    - *listens
                        .get_oldest_listen()
                        .expect("It should have at least one listen")
                        .get_listened_at();

                let rate = ListenRate::new(recording_id, listens.len() as u64, duration);

                (listens, rate)
            })
            .collect_vec())
    }
}
