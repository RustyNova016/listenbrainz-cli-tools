use crate::models::data::listenbrainz::listen::Listen;
use crate::models::data::musicbrainz::mbid::state_id::state::PrimaryMBID;
use crate::models::data::musicbrainz::recording::Recording;
use extend::ext;
use itertools::Itertools;
use std::sync::Arc;

use super::traits::CollectionOfListens;
use super::ListenCollection;

/// Listen with a primary recording ID
pub type PrimaryListen = (PrimaryMBID<Recording>, Arc<Listen>);

pub type PrimaryListenCollection = Vec<PrimaryListen>;

#[ext(name = MappedPrimaryListenCollectionExt)]
pub impl PrimaryListenCollection {
    // --- Querry methods ---

    /// Return only the listens of a specific recording
    fn where_mapped_recording_eq(&self, id: &PrimaryMBID<Recording>) -> Self {
        self.iter()
            .filter_map(|val| {
                if &val.0 == id {
                    Some(val.clone())
                } else {
                    None
                }
            })
            .collect_vec()
    }

    // --- Assertions ---

    // --- Conversions

    fn into_mbids(self) -> Vec<PrimaryMBID<Recording>> {
        self.into_iter().map(|(id, _)| id).collect_vec()
    }

    fn into_legacy(self) -> ListenCollection {
        ListenCollection::new(self.iter_listens().cloned().collect_vec())
    }
}

impl CollectionOfListens for PrimaryListenCollection {
    fn iter_listens(&self) -> impl Iterator<Item = &Arc<Listen>> {
        self.iter().map(|(_, listen)| listen)
    }
}
