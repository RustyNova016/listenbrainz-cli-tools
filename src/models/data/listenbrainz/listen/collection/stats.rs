use std::ops::Deref;
use crate::core::statistics::statistic_sorter::StatisticSorter;
use crate::models::data::listenbrainz::listen::collection::ListenCollection;

impl ListenCollection {
    pub fn get_recording_statistics(&self) -> StatisticSorter {
        let mapped = self.get_mapped_listens();
        
        let counter = StatisticSorter::new();
        
        for listen in mapped {
            counter.insert(listen.clone().get_mapping_data().as_ref().expect("The listen should be mapped").recording_mbid(), listen);
        }
        
        counter
    }
    
    pub async fn get_artist_statistics(&self) -> color_eyre::Result<StatisticSorter> {
        let mapped = self.get_mapped_listens();

        let counter = StatisticSorter::new();

        for listen in mapped {
            let artist_ids = listen.clone().get_mapping_data().as_ref().expect("The listen should be mapped").get_or_fetch_artist_mbids().await?;

            for artist_id in artist_ids {
                counter.insert(artist_id.deref(), listen.clone());
            }
        }

        Ok(counter)
    }

    pub async fn get_release_statistics(&self) -> color_eyre::Result<StatisticSorter> {
        let mapped = self.get_mapped_listens();

        let counter = StatisticSorter::new();

        for listen in mapped {
            let releases_ids = listen.clone().get_mapping_data().as_ref().expect("The listen should be mapped").get_or_fetch_recording().await?.get_or_fetch_artist_credits();

            for artist_id in artist_ids {
                counter.insert(artist_id.deref(), listen.clone());
            }
        }

        Ok(counter)
    }
}