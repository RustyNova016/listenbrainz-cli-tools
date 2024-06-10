use chrono::{TimeZone, Utc};
use listenbrainz::raw::response::UserListensListen;

use crate::models::data::listenbrainz::mapping_data::MappingData;
use crate::models::data::listenbrainz::messybrainz::MessyBrainzData;

use super::Listen;

impl From<UserListensListen> for Listen {
    fn from(value: UserListensListen) -> Self {
        let listened_at = Utc
            .timestamp_opt(value.listened_at, 0)
            .single()
            .expect("Cannot convert listened_at timestamp. This shouldn't happen since all the dates are UTC!");

        Self {
            user: value.user_name.clone(),
            listened_at,
            messybrainz_data: MessyBrainzData::from(value.clone()),
            mapping_data: value
                .track_metadata
                .mbid_mapping
                .clone()
                .map(MappingData::from),
            mapped_recording_id: value
                .track_metadata
                .mbid_mapping
                .clone()
                .map(|data| data.recording_mbid.into()),
        }
    }
}
