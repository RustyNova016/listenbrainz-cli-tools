use std::ops::Deref;
use std::time::Duration;
use indicatif::{ProgressBar, ProgressStyle};
use crate::core::statistics::statistic_sorter::StatisticSorter;
use crate::models::data::listenbrainz::listen::collection::ListenCollection;
use crate::utils::logger::Logger;

impl ListenCollection {
    pub fn get_recording_statistics(&self) -> StatisticSorter {
        let mapped = self.get_mapped_listens();

        let mut progress_bar = ProgressBar::new(mapped.len().try_into().unwrap());
        progress_bar = progress_bar.with_style(ProgressStyle::with_template("[Calculating recording statistics] {wide_bar} {pos}/{len} | {eta_precise}").unwrap());
        progress_bar.enable_steady_tick(Duration::from_secs(1));
        Logger::add_global_pg(progress_bar.clone());

        let counter = StatisticSorter::new();
        for listen in mapped {
            counter.insert(listen.clone().get_mapping_data().as_ref().expect("The listen should be mapped").recording_mbid(), listen);
            progress_bar.inc(1);
        }

        Logger::remove_global_pg(progress_bar);
        counter
    }
    
    pub async fn get_artist_statistics(&self) -> color_eyre::Result<StatisticSorter> {
        let mapped = self.get_mapped_listens();

        let mut progress_bar = ProgressBar::new(mapped.len().try_into().unwrap());
        progress_bar = progress_bar.with_style(ProgressStyle::with_template("[Calculating artist statistics] {wide_bar} {pos}/{len} | {eta_precise}").unwrap());
        progress_bar.enable_steady_tick(Duration::from_secs(1));
        Logger::add_global_pg(progress_bar.clone());

        let counter = StatisticSorter::new();
        for listen in mapped {
            let artist_ids = listen.clone().get_mapping_data().as_ref().expect("The listen should be mapped").get_or_fetch_artist_mbids().await?;

            for artist_id in artist_ids {
                counter.insert(artist_id.deref(), listen.clone());
            }
            progress_bar.inc(1);
        }

        Logger::remove_global_pg(progress_bar);
        Ok(counter)
    }

    pub async fn get_release_statistics(&self) -> color_eyre::Result<StatisticSorter> {
        let mapped = self.get_mapped_listens();

        let mut progress_bar = ProgressBar::new(mapped.len().try_into().unwrap());
        progress_bar = progress_bar.with_style(ProgressStyle::with_template("[Calculating release statistics] {wide_bar} {pos}/{len} | {eta_precise}").unwrap());
        progress_bar.enable_steady_tick(Duration::from_secs(1));
        Logger::add_global_pg(progress_bar.clone());

        let counter = StatisticSorter::new();
        for listen in mapped {
            let releases_ids = listen.clone().get_mapping_data().as_ref().expect("The listen should be mapped").get_or_fetch_recording().await?.get_or_fetch_releases_ids().await?;

            for releases_id in releases_ids {
                counter.insert(releases_id.deref(), listen.clone());
                
            };
            progress_bar.inc(1);
        }

        Logger::remove_global_pg(progress_bar);
        Ok(counter)
    }
}