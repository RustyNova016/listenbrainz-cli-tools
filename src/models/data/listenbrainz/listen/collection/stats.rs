use std::ops::Deref;
use std::sync::Arc;

use itertools::Itertools;
use tokio::task;

use crate::core::display::progress_bar::ProgressBarCli;
use crate::core::entity_traits::mbid::VecIExt;
use crate::core::entity_traits::relations::has_release_group::HasReleaseGroup;
use crate::core::statistics::statistic_sorter::StatisticSorter;
use crate::models::cli::common::GroupByTarget;
use crate::models::data::listenbrainz::listen::collection::ListenCollection;
use crate::models::data::listenbrainz::listen::Listen;

impl ListenCollection {
    pub async fn get_statistics_of(
        &self,
        target: GroupByTarget,
    ) -> color_eyre::Result<Arc<StatisticSorter>> {
        let mapped = self.get_mapped_listens();
        let progress_bar = ProgressBarCli::new(
            mapped.len() as u64,
            Some(&format!("Calculating {} statistics", target.to_str())),
        );

        let counter = Arc::new(StatisticSorter::new());

        let tasks = mapped.into_iter().map(|listen| {
            let counter = counter.clone();
            task::spawn(async move {Self::add_listen_to_target_counter(listen, counter.as_ref(), target).await})});

        for task in tasks {
            task.await??;
        }

        Ok(counter)
    }

    async fn add_listen_to_target_counter(
        listen: Arc<Listen>,
        counter: &StatisticSorter,
        target: GroupByTarget
    ) -> color_eyre::Result<()> {
        match target {
            GroupByTarget::Recording => {
                Self::add_listen_to_recording_counter(listen, counter).await?;
            }
            GroupByTarget::Artist => {
                Self::add_listen_to_artist_counter(listen, counter).await?;
            }
            GroupByTarget::Release => {
                Self::add_listen_to_release_counter(listen, counter).await?;
            }
            GroupByTarget::ReleaseGroup => {
                Self::add_listen_to_release_group_counter(listen, counter).await?;
            }
        }

        Ok(())
    }

    fn get_recording_statistics(self, counter: &StatisticSorter, progress_bar: &ProgressBarCli) {
        for listen in self.into_iter() {
            counter.insert(
                listen
                    .clone()
                    .get_mapping_data()
                    .as_ref()
                    .expect("The listen should be mapped")
                    .recording_mbid(),
                listen,
            );
            progress_bar.inc(1);
        }
    }

    async fn add_listen_to_recording_counter(
        listen: Arc<Listen>,
        counter: &StatisticSorter,
    ) -> color_eyre::Result<()> {
        counter.insert(
            listen
                .clone()
                .get_mapping_data()
                .as_ref()
                .expect("The listen should be mapped")
                .recording_mbid(),
            listen,
        );

        Ok(())
    }

    async fn add_listen_to_artist_counter(
        listen: Arc<Listen>,
        counter: &StatisticSorter,
    ) -> color_eyre::Result<()> {
        let artist_ids = listen
            .clone()
            .get_mapping_data()
            .as_ref()
            .expect("The listen should be mapped")
            .get_or_fetch_artist_mbids()
            .await?;

        for artist_id in artist_ids {
            counter.insert(artist_id.deref(), listen.clone());
        }

        Ok(())
    }

    async fn add_listen_to_release_counter(
        listen: Arc<Listen>,
        counter: &StatisticSorter,
    ) -> color_eyre::Result<()> {
        let releases_ids = listen
            .clone()
            .get_mapping_data()
            .as_ref()
            .expect("The listen should be mapped")
            .get_or_fetch_recording()
            .await?
            .get_or_fetch_releases_ids()
            .await?;

        for releases_id in releases_ids {
            counter.insert(releases_id.deref(), listen.clone());
        }

        Ok(())
    }

    async fn add_listen_to_release_group_counter (
        listen: Arc<Listen>,
        counter: &StatisticSorter,
    ) -> color_eyre::Result<()> {
        let releases = listen
            .clone()
            .get_mapping_data()
            .as_ref()
            .expect("The listen should be mapped")
            .get_or_fetch_recording()
            .await?
            .get_or_fetch_releases_ids()
            .await?
            .get_or_fetch_entities()
            .await?;

        let mut release_groups_ids = Vec::new();
        for release in releases {
            release_groups_ids.push(release.get_or_fetch_release_group().await?);
        }

        release_groups_ids = release_groups_ids.into_iter().unique().collect_vec();

        for release_groups_id in release_groups_ids {
            counter.insert(release_groups_id.deref(), listen.clone());
        }

        Ok(())
    }

    pub async fn get_artist_statistics(
        self,
        counter: &StatisticSorter,
        progress_bar: &ProgressBarCli,
    ) -> color_eyre::Result<()> {
        for listen in self {
            let artist_ids = listen
                .clone()
                .get_mapping_data()
                .as_ref()
                .expect("The listen should be mapped")
                .get_or_fetch_artist_mbids()
                .await?;

            for artist_id in artist_ids {
                counter.insert(artist_id.deref(), listen.clone());
            }
            progress_bar.inc(1);
        }
        Ok(())
    }

    pub async fn get_release_statistics(
        self,
        counter: &StatisticSorter,
        progress_bar: &ProgressBarCli,
    ) -> color_eyre::Result<()> {
        for listen in self {
            let releases_ids = listen
                .clone()
                .get_mapping_data()
                .as_ref()
                .expect("The listen should be mapped")
                .get_or_fetch_recording()
                .await?
                .get_or_fetch_releases_ids()
                .await?;

            for releases_id in releases_ids {
                counter.insert(releases_id.deref(), listen.clone());
            }
            progress_bar.inc(1);
        }

        Ok(())
    }

    pub async fn get_release_group_statistics(
        self,
        counter: &StatisticSorter,
        progress_bar: &ProgressBarCli,
    ) -> color_eyre::Result<()> {
        for listen in self {
            let releases = listen
                .clone()
                .get_mapping_data()
                .as_ref()
                .expect("The listen should be mapped")
                .get_or_fetch_recording()
                .await?
                .get_or_fetch_releases_ids()
                .await?
                .get_or_fetch_entities()
                .await?;

            let mut release_groups_ids = Vec::new();
            for release in releases {
                release_groups_ids.push(release.get_or_fetch_release_group().await?);
            }

            release_groups_ids = release_groups_ids.into_iter().unique().collect_vec();

            for release_groups_id in release_groups_ids {
                counter.insert(release_groups_id.deref(), listen.clone());
            }
            progress_bar.inc(1);
        }

        Ok(())
    }
}
