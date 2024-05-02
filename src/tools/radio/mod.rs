use clap::builder::TypedValueParser;
use color_eyre::eyre::Ok;
use itertools::Itertools;
use listenbrainz::raw::Client;
use rand::{prelude::SliceRandom, thread_rng};

use crate::{
    core::entity_traits::fetchable::FetchableAndCachable, models::data::{
        listenbrainz::{
            listen::{collection::ListenCollection, Listen},
            user_listens::UserListens,
        },
        musicbrainz::{
            artist::Artist,
            recording::Recording,
        },
    }, utils::playlist::PlaylistStub
};

pub async fn create_radio_mix(username: &str, token: String) {
    let listens = UserListens::get_user_with_refresh(username)
        .await
        .expect("Couldn't fetch the new listens")
        .get_mapped_listens();

    let mut tracks = Vec::new();
    for i in (0..10) {
        tracks.push(
            listens
                .choose(&mut rand::thread_rng())
                .and_then(|listen| listen.get_mapping_data().as_ref())
                .map(|data| data.recording_mbid.clone())
                .unwrap_or("".to_string()),
        );
    }

    let radio = RadioCircle {unlistened: true};

    //println!("{}", serde_json::to_string_pretty(&playlist.clone().into_jspf()).unwrap() );
    Client::new()
        .playlist_create(&token, radio.get_playlist(username, &listens).await.expect("Couldn't generate playlist").into_jspf())
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
    ) -> color_eyre::Result<Option<Recording>> {
        let mut recordings = artist.get_all_recordings().await?;

        if self.unlistened {
            recordings = recordings
                .into_iter()
                .filter(|recording| !listens.has_recording(&recording.id))
                .collect_vec();
        }

        recordings.shuffle(&mut thread_rng());

        Ok(recordings.get(0).cloned())
    }

    async fn get_recording_from_listen(&self, listen: &Listen, listens: &ListenCollection) -> color_eyre::Result<Option<Recording>> {
        let Some(mapping_data) = listen.get_mapping_data() else {return Ok(None);};
        let recording = Recording::get_cached_or_fetch(&mapping_data.get_recording_id()).await?;

        for artist_id in recording.get_or_fetch_artist_credits().await?.iter() {
            let artist = Artist::get_cached_or_fetch(&artist_id.artist).await?;

            let result = self.get_recording_of_artist(artist, listens).await?;
            
            if let Some(recording) = result {
                return Ok(Some(recording));
            }
        }

        return Ok(None);
    }

    async fn create_list(&self, listens: &ListenCollection) -> color_eyre::Result<Vec<Recording>> {
        let mut results = Vec::new();

        for listen in listens.iter() {
            let result = self.get_recording_from_listen(listen.as_ref(), &listens).await?;

            if let Some(recording) = result {
                results.push(recording);
            }

            if results.len() > 50 {
                return Ok(results);
            }
        }

        Ok(results)
    }

    pub async fn get_playlist(&self, username: &str, listens: &ListenCollection) -> color_eyre::Result<PlaylistStub> {
        Ok(PlaylistStub::new(
            "Radio Mix".to_string(),
            Some(username.to_string()),
            false,
            self.create_list(listens).await?.into_iter().map(|recording| recording.id).collect_vec(),
        ))
    }
}
