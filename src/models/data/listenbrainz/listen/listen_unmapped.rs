use chrono::DateTime;
use chrono::Utc;

use crate::models::data::listenbrainz::messybrainz::MessyBrainzData;

use super::listen_spe::ListenSpe;
use super::listen_spe::Unmapped;

impl ListenSpe<Unmapped> {
    pub fn new_unmapped(
        username: String,
        listened_at: DateTime<Utc>,
        messybrainz_data: MessyBrainzData,
    ) -> Self {
        Self {
            user: username,
            listened_at,
            messybrainz_data,
            mapping_data: Unmapped {},
        }
    }
}
