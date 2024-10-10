use chrono::Duration;
use itertools::Itertools;

use crate::core::statistics::listen_rate::ListenRate;
use crate::models::cli::common::GroupByTarget;

use super::ListenCollection;

impl ListenCollection {
    pub async fn get_listen_rates(&self) -> color_eyre::Result<Vec<(Self, ListenRate)>> {
        let stats = self.get_statistics_of(GroupByTarget::Recording).await?;

        Ok(stats
            .into_vec()
            .into_iter()
            .map(|(recording_id, listens)| {
                let rate = ListenRate::new(
                    recording_id.into(), //TODO: Use MBID
                    listens.len() as u64,
                    get_duration(&listens),
                );

                (listens, rate)
            })
            .collect_vec())
    }
}

fn get_duration(listens: &ListenCollection) -> Duration {
    // If the recording haven't been listened to, then the average time is zero
    if listens.len() < 2 {
        return Duration::zero();
    }

    *listens
        .get_latest_listen()
        .expect("It should have at least one listen")
        .get_listened_at()
        - *listens
            .get_oldest_listen()
            .expect("It should have at least one listen")
            .get_listened_at()
}
