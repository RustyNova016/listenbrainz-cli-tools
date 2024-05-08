use color_eyre::eyre::Ok;
use itertools::Itertools;
use listenbrainz::raw::Client;
use rand::prelude::SliceRandom;
use rand::thread_rng;
use std::ops::Deref;

use crate::core::entity_traits::cached::Cached;
use crate::core::entity_traits::fetchable::FetchableAndCachable;
use crate::models::data::listenbrainz::listen::collection::ListenCollection;
use crate::models::data::listenbrainz::listen::Listen;
use crate::models::data::listenbrainz::user_listens::UserListens;
use crate::models::data::musicbrainz::artist::Artist;
use crate::models::data::musicbrainz::recording::Recording;
use crate::utils::playlist::PlaylistStub;

pub async fn create_radio_mix(username: &str, token: String, unlistened: bool) {
    let listens = UserListens::get_user_with_refresh(username)
        .await
        .expect("Couldn't fetch the new listens")
        .get_mapped_listens();

    let radio = RadioCircle { unlistened };

    Client::new()
        .playlist_create(
            &token,
            radio
                .get_playlist(username, &listens)
                .await
                .expect("Couldn't generate playlist")
                .into_jspf(),
        )
        .expect("Couldn't send the playlist");
}

pub struct RadioCircle {
    unlistened: bool,
}

impl RadioCircle {
    async fn get_recording_of_artist(
        &self,
        mut artist: Artist,
        listens: &ListenCollection,
        playlist: &[Recording],
    ) -> color_eyre::Result<Option<Recording>> {
        let mut recordings = artist.get_all_recordings().await?;

        if self.unlistened {
            recordings = recordings
                .into_iter()
                .filter(|recording| {
                    !listens.has_recording(&recording.id) && !playlist.contains(recording)
                })
                .collect_vec();
        }

        recordings.shuffle(&mut thread_rng());

        Ok(recordings.first().cloned())
    }

    async fn get_recording_from_listen(
        &self,
        listen: &Listen,
        listens: &ListenCollection,
        playlist: &[Recording],
    ) -> color_eyre::Result<Option<Recording>> {
        let Some(mapping_data) = listen.get_mapping_data() else {
            return Ok(None);
        };
        let recording = Recording::get_cached_or_fetch(mapping_data.recording_mbid()).await?;

        for artist_id in recording.get_or_fetch_artist_credits().await?.iter() {
            let artist = Artist::get_cache().get_or_fetch(&artist_id.artist()).await?;

            let result = self
                .get_recording_of_artist(artist, listens, playlist)
                .await?;

            if let Some(recording) = result {
                return Ok(Some(recording));
            }
        }

        Ok(None)
    }

    async fn create_list(&self, listens: &ListenCollection) -> color_eyre::Result<Vec<Recording>> {
        let mut results = Vec::new();

        let mut listen_shuffle = listens.deref().clone();
        listen_shuffle.shuffle(&mut thread_rng());
        for listen in &listen_shuffle {
            let result = self
                .get_recording_from_listen(listen.as_ref(), listens, &results)
                .await?;

            if let Some(recording) = result {
                results.push(recording);
            }

            if results.len() > 50 {
                return Ok(results);
            }
        }

        Ok(results)
    }

    pub async fn get_playlist(
        &self,
        username: &str,
        listens: &ListenCollection,
    ) -> color_eyre::Result<PlaylistStub> {
        Ok(PlaylistStub::new(
            "Radio Mix".to_string(),
            Some(username.to_string()),
            false,
            self.create_list(listens)
                .await?
                .into_iter()
                .map(|recording| recording.id)
                .collect_vec(),
        ))
    }
}
