use futures::stream;
use futures::StreamExt;

use crate::core::entity_traits::mbid::IsMbid;
use crate::models::data::listenbrainz::user_listens::UserListens;
use crate::models::data::musicbrainz::mbid::extensions::VecATExt;
use crate::models::radio::RadioConfig;
use crate::utils::playlist::PlaylistStub;

pub async fn underrated_mix(
    username: String,
    token: String,
    config: RadioConfig,
) -> color_eyre::Result<()> {
    // Get the listens
    let scores = UserListens::get_user_with_refresh(&username)
        .await
        .expect("Couldn't fetch the new listens")
        .get_listens()
        .get_underrated_recordings()
        .await
        .expect("Couldn't calculate the underrated listens");

    let scores_as_recording = stream::iter(scores.clone())
        .map(|(_, id)| async move { id.get_or_fetch_entity().await })
        .buffered(1);
    let playlist = config.finalize_radio_playlist(scores_as_recording).await?;

    PlaylistStub::new(
        format!("{username}'s underrated mix"),
        Some(username.clone()),
        true,
        playlist.into_mbids(),
        Some(format!("A playlist containing all the tracks that {username} listen to, 
            but seemingly no one else does. Come take a listen if you want to find hidden gems!<br>
            <br>
            The mix is made by calculating a score for each listen. This score is composed of two values:<br>
            - The rank in {username}'s top 1000 recording of all time (First place get 100 points, second get 999.9, etc...)<br>
            - The percentage of the recording's listens being from {username} (Made with this formula: (user listens / worldwide listens) *100)<br>
            <br>
            Made with: https://github.com/RustyNova016/listenbrainz-cli-tools"
        )),
    ).send(&token).await?;

    Ok(())
}
