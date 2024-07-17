use std::hash::Hash;
use std::sync::Arc;

use extend::ext;
use futures::Stream;
use futures::StreamExt;

use crate::core::data_structures::table_map::table_index::TableItem;
use crate::models::data::listenbrainz::listens_with_entity::map::ListensWithEntityMap;
use crate::models::data::musicbrainz::mbid::state_id::state::PrimaryMBID;
use crate::models::data::musicbrainz::recording::Recording;

use super::PrimaryListen;



#[ext(name = TryCollectListensWithEntity)]
pub impl<St, Ent, K, Err> St
where
    St: Stream<Item = Result<(Arc<Ent>, PrimaryListen), Err>> + Unpin,
    K: Eq + Hash,
    Ent: TableItem<K>,
{
    /// Collect a stream of `Result<(Entity, Listens), E>` into a map of [`ListensWithEntity`]
    async fn try_collect_by_entity(&mut self) -> Result<ListensWithEntityMap<K, Ent>, Err> {
        let mut map = ListensWithEntityMap::default();

        while let Some(value) = self.next().await {
            let (entity, listen) = value?;

            map.add_listen(entity, listen);
        }

        Ok(map)
    }
}
