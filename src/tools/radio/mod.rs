use crate::{models::data::listenbrainz::user_listens::UserListens, utils::playlist::PlaylistStub};
use clap::builder::TypedValueParser;
use listenbrainz::raw::{jspf::{Playlist, PlaylistInfo}, Client};
use rand::prelude::SliceRandom;

pub async fn create_radio_mix(username: &str, token: String) {
    let listens = UserListens::get_user_with_refresh(username)
        .await
        .expect("Couldn't fetch the new listens")
        .get_mapped_listens();

    let mut tracks = Vec::new();
    for i in (0..10) {
        tracks.push(listens.choose(&mut rand::thread_rng()).and_then(|listen| listen.get_mapping_data().as_ref()).map(|data| data.recording_mbid.clone()).unwrap_or("".to_string()));
    }

    let playlist = PlaylistStub::new("Radio Mix".to_string(), Some(username.to_string()), false, tracks);
    //println!("{}", serde_json::to_string_pretty(&playlist.clone().into_jspf()).unwrap() );
    Client::new().playlist_create(&token, playlist.into_jspf()).expect("Couldn't send the playlist");
}