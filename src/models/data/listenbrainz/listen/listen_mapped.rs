use chrono::DateTime;
use chrono::Utc;

use crate::core::entity_traits::mbid::is_cached_mbid::IsCachedMBID;
use crate::core::entity_traits::mbid::IsMbid;
use crate::models::data::listenbrainz::mapping_data::MappingData;
use crate::models::data::listenbrainz::messybrainz::MessyBrainzData;
use crate::models::data::musicbrainz::recording::mbid::RecordingMBID;
use crate::models::data::musicbrainz::recording::Recording;

use super::listen_spe::ListenSpe;

impl ListenSpe<MappingData> {
    pub fn new_mapped(
        username: String,
        listened_at: DateTime<Utc>,
        messybrainz_data: MessyBrainzData,
        mapping_data: MappingData,
    ) -> Self {
        Self {
            user: username,
            listened_at,
            messybrainz_data,
            mapping_data,
        }
    }

    /// If mapped, return the recording MBID
    pub fn get_recording_mbid_as_string(&self) -> &String {
        &self.mapping_data.recording_mbid
    }

    pub fn get_naive_recording_mbid(&self) -> RecordingMBID {
        self.mapping_data.recording_mbid.clone().into()
    }

    pub async fn get_recording_mbid(&self) -> color_eyre::Result<RecordingMBID> {
        self.mapping_data
            .get_recording_mbid()
            .get_or_fetch_primary_mbid_alias()
            .await
    }

    /// Return the recording's data from Musicbrainz from its mapping
    pub async fn get_recording_data(&self) -> color_eyre::Result<Recording> {
        self.get_naive_recording_mbid().get_or_fetch_entity().await
    }
}
