use chrono::DateTime;
use chrono::Utc;

use crate::core::entity_traits::mbid::is_cached_mbid::IsCachedMBID;
use crate::core::entity_traits::mbid::IsMbid;
use crate::models::data::listenbrainz::mapping_data::MappingData;
use crate::models::data::listenbrainz::messybrainz::MessyBrainzData;
use crate::models::data::musicbrainz::mbid::generic_mbid::MBIDSpe;
use crate::models::data::musicbrainz::mbid::generic_mbid::NaiveID;
use crate::models::data::musicbrainz::recording::mbid::RecordingMBID;
use crate::models::data::musicbrainz::recording::Recording;

use super::listen_spe::ListenSpe;
use super::listen_spe::MappedPrimary;
use super::mapped_primary::MappedListen;
use super::Listen;

pub type NaiveMappedListen = ListenSpe<MappingData>;

impl NaiveMappedListen {
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

    pub fn get_legacy_recording_mbid(&self) -> RecordingMBID {
        self.mapping_data.recording_mbid.clone().into()
    }

    pub fn get_naive_recording_mbid(&self) -> MBIDSpe<Recording, NaiveID> {
        self.mapping_data.recording_mbid.clone().into()
    }

    #[deprecated]
    pub async fn get_recording_mbid(&self) -> color_eyre::Result<RecordingMBID> {
        self.mapping_data
            .get_recording_mbid()
            .get_or_fetch_primary_mbid_alias()
            .await
    }

    /// Return the recording's data from Musicbrainz from its mapping
    pub async fn get_recording_data(&self) -> color_eyre::Result<Recording> {
        self.get_legacy_recording_mbid().get_or_fetch_entity().await
    }

    pub async fn into_primary_mapping(&self) -> color_eyre::Result<MappedListen> {
        let new_id = self.get_naive_recording_mbid().into_primary().await?;

        Ok(MappedListen {
            listened_at: self.listened_at,
            mapping_data: new_id,
            messybrainz_data: self.messybrainz_data.clone(),
            user: self.user.clone(),
        })
    }

    fn into_legacy(self) -> Listen {
        self.into()
    }
}

impl From<NaiveMappedListen> for Listen {
    fn from(value: NaiveMappedListen) -> Self {
        Listen {
            listened_at: value.listened_at,
            mapping_data: Some(value.mapping_data),
            user: value.user,
            messybrainz_data: value.messybrainz_data
        }
    }
}