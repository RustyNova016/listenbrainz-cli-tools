pub mod group_by;
use crate::models::data::listenbrainz::listen::primary_listen::PrimaryListen;
use crate::models::data::listenbrainz::listen::Listen;
use crate::models::data::listenbrainz::listens_with_entity::map::ListensWithEntityMap;
use crate::models::data::musicbrainz::artist::Artist;
use crate::models::data::musicbrainz::mbid::state_id::state::PrimaryMBID;
use crate::models::data::musicbrainz::recording::Recording;
use extend::ext;
use futures::stream;
use futures::StreamExt;
use futures::TryStreamExt;
use itertools::Itertools;
use std::sync::Arc;
use tokio::sync::RwLock;

use super::traits::CollectionOfListens;
use super::ListenCollection;

pub type PrimaryListenCollection = Vec<Arc<PrimaryListen>>;

#[ext(name = MappedPrimaryListenCollectionExt)]
pub impl PrimaryListenCollection {
    // --- Querry methods ---

    /// Return only the listens of a specific recording
    fn where_mapped_recording_eq(&self, id: &PrimaryMBID<Recording>) -> Self {
        self.iter()
            .filter(|val| &val.get_mbid() == id)
            .cloned()
            .collect_vec()
    }

    // --- Assertions ---

    // --- Conversions

    fn into_mbids(self) -> Vec<PrimaryMBID<Recording>> {
        self.into_iter()
            .map(|listen| listen.get_mbid())
            .collect_vec()
    }

    fn into_legacy(self) -> ListenCollection {
        ListenCollection::new(self.iter_listens().cloned().collect_vec())
    }

    // --- Stats ---
    fn map_mapped_recordings(&self) -> ListensWithEntityMap<PrimaryMBID<Recording>, Recording> {
        let mut map = ListensWithEntityMap::default();

        for listen in self {
            map.add_listen(listen.mapped_recording().clone(), listen.clone());
        }

        map
    }

    async fn map_mapped_artists(
        &self,
    ) -> color_eyre::Result<ListensWithEntityMap<PrimaryMBID<Artist>, Artist>> {
        let map = RwLock::new(ListensWithEntityMap::default());

        //TODO: Use Stream
        for listen in self {
            ListensWithEntityMap::add_listen_artist_credits(&map, listen.clone()).await?;
        }

        Ok(map.into_inner())
    }
}

impl CollectionOfListens for PrimaryListenCollection {
    fn iter_listens(&self) -> impl Iterator<Item = &Arc<Listen>> {
        self.iter().map(|listen| listen.listen())
    }
}
