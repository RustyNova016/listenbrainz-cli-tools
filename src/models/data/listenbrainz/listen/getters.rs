use crate::core::entity_traits::mbid::is_cached_mbid::IsCachedMBID;
use crate::models::data::listenbrainz::messybrainz::MessyBrainzData;
use crate::models::data::musicbrainz::recording::mbid::RecordingMBID;
use crate::Error;

use super::Listen;

impl Listen {
    pub fn get_messybrain_data(&self) -> &MessyBrainzData {
        &self.messybrainz_data
    }

    pub async fn get_primary_recording_id(&self) -> Result<Option<RecordingMBID>, Error> {
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

    pub fn get_naive_recording_mbid(&self) -> Option<RecordingMBID> {
        self.mapping_data
            .as_ref()
            .map(|val| val.get_recording_mbid())
    }
}
