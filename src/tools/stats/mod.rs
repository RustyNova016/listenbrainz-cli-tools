use crate::models::cli::common::GroupByTarget;

mod artists;
mod recordings;
mod releases;

pub async fn stats_command(username: &str, target: GroupByTarget) {
    match target {
        GroupByTarget::Recording => {
            recordings::stats_recording(username).await;
        }
        GroupByTarget::Artist => {
            artists::stats_artist(username).await;
        }
        GroupByTarget::Release => {
            releases::stats_releases(username).await;
        }
    }
}
