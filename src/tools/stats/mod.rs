use crate::models::cli::stats::GroupByTarget;

mod artists;
mod recordings;

pub fn stats_command(username: &str, target: GroupByTarget) {
    match target {
        GroupByTarget::Recording => {
            recordings::stats_recording(username);
        }
        GroupByTarget::Artist => {
            artists::stats_artist(username);
        }
    }
}
