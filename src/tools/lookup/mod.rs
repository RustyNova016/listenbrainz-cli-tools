pub mod recording;
use crate::models::data::musicbrainz::mbid::state_id::any::any_entity::AnyEntityMBID;
use crate::models::data::musicbrainz::mbid::state_id::state::NaiveIDState;
use recording::lookup_recording;

pub async fn lookup_command(
    username: &str,
    id: AnyEntityMBID<NaiveIDState>,
) -> color_eyre::Result<()> {
    match id {
        AnyEntityMBID::Recording(val) => lookup_recording(username, val).await,
        _ => todo!(),
    }
}
