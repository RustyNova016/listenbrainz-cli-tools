use crate::models::data::listenbrainz::messybrainz::MessyBrainzData;

use super::Listen;

impl Listen {
    pub fn get_messybrain_data(&self) -> &MessyBrainzData {
        &self.messybrainz_data
    }
}
