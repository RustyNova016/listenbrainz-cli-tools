use super::listen_spe::ListenSpe;
use super::listen_unspe::ListenMappingState;
use super::Listen;
use crate::models::data::listenbrainz::mapping_data::MappingData;
use crate::models::data::listenbrainz::messybrainz::MessyBrainzData;
use chrono::{TimeZone, Utc};
use listenbrainz::raw::response::UserListensListen;

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
            mapping_data: value.track_metadata.mbid_mapping.map(MappingData::from),
        }
    }
}

impl From<Listen> for ListenMappingState {
    fn from(value: Listen) -> Self {
        if value.is_mapped() {
            ListenSpe::new_mapped(
                value.user,
                value.listened_at,
                value.messybrainz_data,
                value.mapping_data.unwrap(),
            )
            .into()
        } else {
            ListenSpe::new_unmapped(value.user, value.listened_at, value.messybrainz_data).into()
        }
    }
}
