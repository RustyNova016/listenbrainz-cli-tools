use std::sync::Arc;

use derive_getters::Getters;

use crate::models::data::musicbrainz::entity::traits::MusicBrainzEntity;
use crate::models::data::musicbrainz::mbid::state_id::state::PrimaryMBID;
use crate::models::data::musicbrainz::recording::Recording;

use super::Listen;

pub mod stream;

#[derive(Clone, PartialEq, Eq, Debug, Getters)]
pub struct PrimaryListen {
    listen: Arc<Listen>,
    mapped_recording: Arc<Recording>,
}

impl PrimaryListen {
    pub async fn from_listen(listen: Arc<Listen>) -> Option<color_eyre::Result<Self>> {
        Some(
            listen
                .get_load_or_fetch_recording()
                .await?
                .map(|recording| Self {
                    mapped_recording: recording,
                    listen,
                }),
        )
    }

    pub fn get_mbid(&self) -> PrimaryMBID<Recording> {
        self.mapped_recording.get_mbid()
    }
}
