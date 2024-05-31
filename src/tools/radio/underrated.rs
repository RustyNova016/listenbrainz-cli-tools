use itertools::Itertools;
use listenbrainz::raw::Client;

use crate::models::data::listenbrainz::user_listens::UserListens;
use crate::utils::playlist::PlaylistStub;

pub async fn underrated_mix(username: String, token: String) {
    // Get the listens
    let scores = UserListens::get_user_with_refresh(&username)
        .await
        .expect("Couldn't fetch the new listens")
        .get_listens()
        .get_underrated_recordings()
        .await
        .expect("Couldn't calculate the underrated listens");

    let chunked = scores.chunks(50).collect_vec();
    let bests = chunked
        .first()
        .expect("No recordings have been listened to");

    let playlist_stub = PlaylistStub::new(
        format!("{username}'s underrated mix"),
        Some(username.clone()),
        true,
        bests.iter().map(|x| x.1.clone()).collect_vec(),
        Some(format!("A playlist containing all the tracks that {username} listen to, 
            but seemingly no one else does. Come take a listen if you want to find hidden gems!<br>
            <br>
            The mix is made by calculating a score for each listen. This score is composed of two values:<br>
            - The rank in {username}'s top 1000 recording of all time (First place get 100 points, second get 999.9, etc...)<br>
            - The percentage of the recording's listens being from {username} (Made with this formula: (user listens / worldwide listens) *100)<br>
            <br>
            Made with: https://github.com/RustyNova016/listenbrainz-cli-tools"
        )),
    );

    Client::new()
        .playlist_create(&token, playlist_stub.into_jspf())
        .expect("Couldn't send the playlist");
}
