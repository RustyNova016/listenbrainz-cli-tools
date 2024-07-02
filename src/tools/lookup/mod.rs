pub mod recording;
use crate::models::data::musicbrainz::mbid::MBID;
use recording::lookup_recording;

pub async fn lookup_command(username: &str, id: MBID) -> color_eyre::Result<()> {
    match id {
        MBID::Recording(val) => lookup_recording(username, val).await,
        _ => todo!(),
    }
}
