use crate::{
    models::data::listens::{collection::UserListenCollection, UserListen},
    utils::extensions::UserListensPayloadExt,
};
use chrono::{DateTime, Utc};
use listenbrainz::raw::response::{UserListensListen, UserListensPayload};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub mod cache;

/// All of a user's listens
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserListensCache {
    username: String,
    listens: Vec<UserListenCache>,
}

impl UserListensCache {
    pub fn new(username: String) -> Self {
        Self {
            username,
            listens: Vec::new(),
        }
    }

    /// Remove all the listens in a specific timerange
    fn invalidate_timerange(&mut self, start: DateTime<Utc>, end: DateTime<Utc>) {
        self.listens.retain(|listen| {
            listen.listen_data.listened_at < start || end < listen.listen_data.listened_at
        });
    }

    pub fn insert_api_return(&mut self, data: UserListensPayload) {
        self.invalidate_timerange(
            data.get_date_of_oldest_listen_of_payload()
                .unwrap_or(Utc::now()),
            data.get_date_of_latest_listen_of_payload()
                .unwrap_or(Utc::now()),
        );

        for new_listen in data.listens {
            self.listens.push(new_listen.into())
        }
    }

    /// Return the listen with the latest listen date from the cache
    pub fn get_latest_cached_listen(&self) -> Option<&UserListenCache> {
        self.listens
            .iter()
            .max_by_key(|listen| listen.listen_data.listened_at)
    }

    pub fn get_listens(&self) -> UserListenCollection {
        UserListenCollection::from_iter(
            self.listens
                .iter()
                .map(|cached_listen| cached_listen.listen_data.clone()),
        )
    }
}

/// An holder for a Listen with caching info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserListenCache {
    pub listen_data: Arc<UserListen>,
    pub updated_at: DateTime<Utc>,
}

impl UserListenCache {
    pub fn new(listen_data: Arc<UserListen>) -> Self {
        Self {
            listen_data,
            updated_at: chrono::offset::Utc::now(),
        }
    }
}

impl From<UserListensListen> for UserListenCache {
    fn from(value: UserListensListen) -> Self {
        Self::new(Arc::new(
            UserListen::try_from(value).expect("Couldn't parse timestamp of listen"),
        ))
    }
}
