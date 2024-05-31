use crate::core::entity_traits::mbid::is_cached_mbid::IsCachedMBID;
use crate::models::data::listenbrainz::messybrainz::MessyBrainzData;
use crate::models::data::musicbrainz::recording::mbid::RecordingMBID;

use super::Listen;

impl Listen {
    pub fn get_messybrain_data(&self) -> &MessyBrainzData {
        &self.messybrainz_data
    }

    pub async fn get_primary_recording_id(&self) -> color_eyre::Result<Option<RecordingMBID>> {
        let Some(mapping) = self.mapping_data.as_ref() else {
            return Ok(None);
        };

        Ok(Some(
            mapping
                .get_recording_mbid()
                .get_or_fetch_primary_mbid_alias()
                .await?,
        ))
    }
}
