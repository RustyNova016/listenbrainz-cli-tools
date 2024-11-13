use crate::database::get_conn;
use crate::database::listenbrainz::listens::ListenFetchQuery;
use crate::database::listenbrainz::listens::ListenFetchQueryReturn;
use crate::models::cli::common::GroupByTarget;
use crate::models::cli::common::SortSorterBy;

mod artists;
mod recordings;
mod release_groups;
mod releases;
mod work;

pub async fn stats_command(username: &str, target: GroupByTarget, _sort_by: SortSorterBy) {
    let listens = ListenFetchQuery::builder()
        //.fetch_recordings_redirects(true)
        .returns(ListenFetchQueryReturn::Mapped)
        .user(username.to_string())
        .build()
        .fetch(&mut *get_conn().await)
        .await
        .expect("Couldn't fetch the new listens");

    match target {
        GroupByTarget::Recording => {
            recordings::stats_recording(listens).await;
        }
        GroupByTarget::Artist => {
            //artists::stats_artist(stats, sort_by).await;
            todo!()
        }
        GroupByTarget::Release => {
            releases::stats_releases(listens).await;
        }
        GroupByTarget::ReleaseGroup => {
            release_groups::stats_release_groups(listens).await;
        }
        GroupByTarget::Work => {
            work::stats_works(listens).await;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::println_cli_info;

    use super::*;

    #[tokio::test]
    #[serial_test::serial]
    async fn stats_command_recordings() {
        //let mut clog = colog::default_builder();
        //clog.filter(None, log::LevelFilter::Trace);
        //clog.init();

        println_cli_info("--- Starting test ---");
        stats_command("RustyNova", GroupByTarget::Recording, SortSorterBy::Count).await;
    }
}
