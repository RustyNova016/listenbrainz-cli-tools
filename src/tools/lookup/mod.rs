pub mod recording;
use crate::models::cli::lookup::LookupTarget;
use recording::lookup_recording;

pub async fn lookup_command(
    username: &str,
    id: &str,
    target: LookupTarget,
) -> color_eyre::Result<()> {
    match target {
        LookupTarget::Recording => lookup_recording(username, id).await,
    }
}
