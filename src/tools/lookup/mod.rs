pub mod recording;
use crate::models::data::musicbrainz::mbid::MBIDEnum;
use recording::lookup_recording;

pub async fn lookup_command(username: &str, id: MBIDEnum) -> color_eyre::Result<()> {
    match id {
        MBIDEnum::Recording(val) => lookup_recording(username, val).await,
        _ => todo!(),
    }
}
