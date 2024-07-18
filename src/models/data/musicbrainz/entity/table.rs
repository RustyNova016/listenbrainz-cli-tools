use crate::core::data_structures::table_map::table_index::TableItem;
use crate::models::data::musicbrainz::mbid::state_id::state::PrimaryMBID;

use super::traits::MusicBrainzEntity;

impl<E> TableItem<PrimaryMBID<E>> for E
where
    E: MusicBrainzEntity,
{
    fn get_key(&self) -> PrimaryMBID<E> {
        self.get_mbid()
    }
}
