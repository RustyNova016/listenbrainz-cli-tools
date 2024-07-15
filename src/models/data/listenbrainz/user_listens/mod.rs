pub mod import;
use std::sync::Arc;

use chrono::{DateTime, Utc};
use derive_getters::Getters;
use listenbrainz::raw::response::{UserListensListen, UserListensPayload};
use serde::{Deserialize, Serialize};

use crate::utils::extensions::UserListensPayloadExt;

use super::listen::collection::mapped_primary_collection::PrimaryListenCollection;
use super::listen::collection::ListenCollection;
use super::listen::Listen;

pub mod caching;
pub mod fetching;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Getters)]
pub struct UserListens {
    username: String,
    #[getter(rename = "get_listens")]
    listens: ListenCollection,
}

impl UserListens {
    pub fn new(user: &str) -> Self {
        Self {
            username: user.to_lowercase(),
            listens: ListenCollection::default(),
        }
    }

    pub fn get_user(&self) -> &str {
        &self.username
    }

    pub fn get_latest_listen(&self) -> Option<Arc<Listen>> {
        self.listens.get_latest_listen()
    }

    /// Insert a listen into the struct.
    ///
    /// ⚠️ This doesn't affect the cache ⚠️
    pub fn insert_listen(&mut self, listen: Listen) {
        self.listens.push(Arc::new(listen));
    }

    /// Remove all the listens in a specific timerange and replace them with payload data.
    ///
    /// ⚠️ This doesn't affect the cache ⚠️
    pub fn refresh_timerange(&mut self, data: UserListensPayload) {
        self.remove_timerange(
            &data
                .get_date_of_oldest_listen_of_payload()
                .unwrap_or_else(Utc::now),
            &data
                .get_date_of_latest_listen_of_payload()
                .unwrap_or_else(Utc::now),
            true,
        );

        for lb_listen in data.listens {
            self.insert_lb_listen(lb_listen);
        }
    }

    /// Remove all the listens in a specific timerange. This is a dangerous function as it can mess with data integrity
    ///
    /// ⚠️ This doesn't affect the cache ⚠️
    pub fn remove_timerange(
        &mut self,
        start: &DateTime<Utc>,
        end: &DateTime<Utc>,
        inclusive: bool,
    ) {
        self.listens.remove_timerange(start, end, inclusive);
    }

    /// Uncached and unchecked insert of a listenbrain listen into the struct
    ///
    /// ⚠️ This doesn't affect the cache ⚠️
    pub fn insert_lb_listen(&mut self, data: UserListensListen) {
        self.listens.push(Arc::new(data.into()));
    }

    /// Returns all the unmapped listens
    pub fn get_unmapped_listens(&self) -> ListenCollection {
        self.listens.get_unmapped_listens()
    }

    /// Returns all the mapped listens
    pub fn get_mapped_listens(&self) -> ListenCollection {
        self.listens.get_mapped_listens()
    }

    pub async fn get_primary_listens(&self) -> color_eyre::Result<PrimaryListenCollection> {
        self.listens.clone().try_into_mapped_primary().await
    }

    /// Returns the number of listens
    pub fn len(&self) -> usize {
        self.listens.len()
    }

    /// Returns true if there is no listens
    pub fn is_empty(&self) -> bool {
        self.listens.is_empty()
    }
}
