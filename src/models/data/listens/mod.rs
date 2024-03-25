use chrono::{DateTime, TimeZone, Utc};
use listenbrainz::raw::response::{UserListensListen, UserListensMBIDMapping};

use crate::models::api::musicbrainz::MusicBrainzAPI;
use crate::models::data::recording::Recording;

pub mod collection;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserListen {
    /// Time of when the listen happened
    listened_at: DateTime<Utc>,

    /// Data that have been sent to listenbrainz durring listen submition
    messybrainz_data: MessyBrainzData,

    /// Data of the mapping
    mapping_data: Option<MappingData>,
}

impl UserListen {
    pub fn is_mapped(&self) -> bool {
        self.mapping_data.is_some()
    }

    pub fn get_mapping_data(&self) -> &Option<MappingData> {
        &self.mapping_data
    }

    /// If mapped, return the recording MBID
    pub fn get_recording_mbid(&self) -> Option<&String> {
        self.mapping_data
            .as_ref()
            .map(|mapping| &mapping.recording_mbid)
    }

    /// Return true if the listen is mapped to this recording MBID
    pub fn is_mapped_to_recording(&self, mbid: &str) -> bool {
        self.mapping_data
            .as_ref()
            .is_some_and(|mapping| mapping.recording_mbid == mbid)
    }

    /// Return the recording's data from Musicbrainz if it is mapped
    pub fn get_recording_data(&self, mb_client: &mut MusicBrainzAPI) -> Option<Recording> {
        self.mapping_data
            .as_ref()
            .map(|mapping| mb_client.get_recording_data(&mapping.recording_mbid))
    }
}

impl TryFrom<UserListensListen> for UserListen {
    type Error = &'static str;

    fn try_from(value: UserListensListen) -> Result<Self, Self::Error> {
        let listened_at = Utc
            .timestamp_opt(value.listened_at, 0)
            .single()
            .ok_or("Cannot convert listened_at timestamp")?;

        Ok(Self {
            listened_at,
            messybrainz_data: MessyBrainzData::from(value.clone()),
            mapping_data: value.track_metadata.mbid_mapping.map(MappingData::from),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessyBrainzData {}

impl From<UserListensListen> for MessyBrainzData {
    fn from(_value: UserListensListen) -> Self {
        Self {}
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MappingData {
    /// The MBID of the recordings
    recording_mbid: String,

    /// Name of the recording
    recording_name: String,

    /// Artists MBID
    artist_mbid: Vec<String>,
}

impl MappingData {
    pub fn get_recording_id(&self) -> &String {
        &self.recording_mbid
    }

    pub fn get_recording_name(&self) -> &String {
        &self.recording_name
    }

    pub fn get_artists_mbids(&self) -> &Vec<String> {
        &self.artist_mbid
    }
}

impl From<UserListensMBIDMapping> for MappingData {
    fn from(value: UserListensMBIDMapping) -> Self {
        Self {
            recording_mbid: value.recording_mbid.clone(),
            recording_name: value
                .recording_name
                .unwrap_or(format!("Unknown Track ({})", value.recording_mbid)),
            artist_mbid: value.artist_mbids.unwrap_or_default(),
        }
    }
}
