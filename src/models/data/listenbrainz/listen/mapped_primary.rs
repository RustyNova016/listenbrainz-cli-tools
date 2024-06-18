use crate::models::data::musicbrainz::mbid::generic_mbid::MBIDSpe;
use crate::models::data::musicbrainz::mbid::generic_mbid::PrimaryID;
use crate::models::data::musicbrainz::recording::Recording;

use super::listen_spe::ListenSpe;
use super::listen_spe::MappedPrimary;

pub type MappedListen = ListenSpe<MappedPrimary>;

impl MappedListen {
    pub fn get_recording_mbid(&self) -> &MBIDSpe<Recording, PrimaryID> {
        &self.mapping_data
    }
}