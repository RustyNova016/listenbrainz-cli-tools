use std::ops::Deref;

use itertools::Itertools;

use crate::core::display::progress_bar::ProgressBarCli;
use crate::core::entity_traits::mbid::VecIExt;
use crate::core::entity_traits::relations::has_release_group::HasReleaseGroup;
use crate::core::statistics::statistic_sorter::StatisticSorter;
use crate::models::cli::common::GroupByTarget;
use crate::models::data::listenbrainz::listen::collection::ListenCollection;

impl ListenCollection {
    pub async fn get_statistics_of(
        &self,
        target: GroupByTarget,
    ) -> color_eyre::Result<StatisticSorter> {
        let mapped = self.get_mapped_listens();
        let progress_bar = ProgressBarCli::new(
            mapped.len() as u64,
            Some(&format!("Calculating {} statistics", target.to_str())),
        );

        let counter = StatisticSorter::new();

        match target {
            GroupByTarget::Recording => {
                mapped.get_recording_statistics(&counter, &progress_bar);
            }
            GroupByTarget::Artist => {
                mapped
                    .get_artist_statistics(&counter, &progress_bar)
                    .await?;
            }
            GroupByTarget::Release => {
                mapped
                    .get_release_statistics(&counter, &progress_bar)
                    .await?;
            }
            GroupByTarget::ReleaseGroup => {
                mapped
                    .get_release_group_statistics(&counter, &progress_bar)
                    .await?;
            }
        }

        Ok(counter)
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
