pub mod artist;
pub mod impl_trait;
pub mod recording;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;

use crate::core::data_structures::table_map::table_index::TableItem;
use crate::models::data::listenbrainz::listen::primary_listen::PrimaryListen;

use super::ListensWithEntity;

pub struct ListensWithEntityMap<K, E>
where
    K: Eq + Hash,
    E: TableItem<K>,
{
    data: HashMap<K, ListensWithEntity<E>>,
}

impl<K, E> ListensWithEntityMap<K, E>
where
    K: Eq + Hash,
    E: TableItem<K>,
{
    pub fn add_listen(&mut self, entity: Arc<E>, listen: Arc<PrimaryListen>) {
        self.data
            .entry(entity.get_key())
            .or_insert_with(|| ListensWithEntity::new_empty(entity.clone()))
            .push(listen);
    }
}
