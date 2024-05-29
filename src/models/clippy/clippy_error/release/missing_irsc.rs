use crate::core::entity_traits::mbid::IsMbid;
use crate::models::clippy::clippy_error::IsClippyError;
use crate::models::data::musicbrainz::mbid::MBID;
use crate::models::data::musicbrainz::recording::Recording;

pub struct MissingISRC {
    recording: Recording,
    recording_string: String,
}

impl IsClippyError for MissingISRC {
    async fn check_for_error(id: MBID) -> color_eyre::Result<Option<Self>> {
        // Check if it is a recording
        let MBID::Recording(id) = id else {
            return Ok(None);
        };

        let recording = id.get_or_fetch_entity().await?;
        // Check if it has ISRCs
        if recording.isrcs().is_some_and(|data| !data.is_empty()) {
            return Ok(None);
        }

        // Check if there is a release with a Spotify URL
        let spotify = recording.is_streamable_on("spotify.com").await?;
        if !spotify {
            return Ok(None);
        }

        let recording_string = recording.get_title_with_credits().await?;
        Ok(Some(Self { recording, recording_string }))
    }

    fn get_title(&self) -> String {
        format!("Missing ISRC for recording: `{}`", self.recording_string)
    }

    fn get_relevant_url(&self) -> String {
        todo!()
    }

    fn get_description(&self) -> String {
        todo!()
    }

    fn get_additions(&self) -> Vec<(String, String)> {
        todo!()
    }
}
