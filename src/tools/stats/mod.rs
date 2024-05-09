use crate::models::cli::common::GroupByTarget;
use crate::tools::stats::release_groups::stats_release_groups;

mod artists;
mod recordings;
mod release_groups;
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
        GroupByTarget::ReleaseGroup => {
            stats_release_groups(username).await;
        }
    }
}
