use crate::models::data::listenbrainz::listen::listen_spe::ListenSpe;
use crate::models::data::listenbrainz::listen::listen_spe::MappedPrimary;
use crate::models::data::musicbrainz::mbid::generic_mbid::MBIDSpe;
use crate::models::data::musicbrainz::mbid::generic_mbid::PrimaryID;
use crate::models::data::musicbrainz::recording::Recording;
use extend::ext;
use itertools::Itertools;
use std::sync::Arc;

use super::ListenCollection;

pub type MappedListenCollection = Vec<Arc<ListenSpe<MappedPrimary>>>;

#[ext]
pub impl MappedListenCollection {
    fn into_recordings_mbids(&self) -> Vec<MBIDSpe<Recording, PrimaryID>> {
        self.iter()
            .map(|listen| listen.get_recording_mbid().clone())
            .collect_vec()
    }

    fn remove_listens_of_mbids(self, blacklist: &[MBIDSpe<Recording, PrimaryID>]) -> Self {
        self.into_iter()
            .filter(|listen| !blacklist.contains(listen.get_recording_mbid()))
            .collect_vec()
    }

    fn into_legacy(self) -> ListenCollection {
        let vec_of_legacy = self
            .into_iter()
            .map(|listen| Arc::new(listen.as_ref().clone().into_legacy()))
            .collect_vec();

        ListenCollection::from_iter(vec_of_legacy)
    }

    fn keep_only_recording(self, id: &MBIDSpe<Recording, PrimaryID>) -> Self {
        self.into_iter()
            .filter(|listen| listen.get_recording_mbid() == id)
            .collect_vec()
    }
}
