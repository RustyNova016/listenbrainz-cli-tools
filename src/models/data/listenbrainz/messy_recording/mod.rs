use crate::models::data::listenbrainz::listen::Listen;
use std::sync::Arc;

/// Represent a messybrain recording id
pub struct MessyRecording {
    pub id: String,
    pub associated_listens: Vec<Arc<Listen>>,
}

impl MessyRecording {
    pub fn new(id: String) -> Self {
        MessyRecording {
            id,
            associated_listens: Vec::new(),
        }
    }

    pub fn add_listen(&mut self, listen: Arc<Listen>) {
        if listen.messybrainz_data.msid == self.id {
            self.associated_listens.push(listen)
        }
    }

    pub fn get_recording_name(&self) -> Option<String> {
        self.associated_listens
            .iter()
            .find(|listen| !listen.messybrainz_data.track_name.is_empty())
            .map(|listen| listen.messybrainz_data.track_name.clone())
    }

    pub fn get_artist_name(&self) -> Option<String> {
        self.associated_listens
            .iter()
            .find(|listen| !listen.messybrainz_data.artist_name.is_empty())
            .map(|listen| listen.messybrainz_data.artist_name.clone())
    }

    pub fn get_latest_listen(&self) -> Option<&Arc<Listen>> {
        self.associated_listens
            .iter()
            .max_by_key(|listen| listen.listened_at)
    }

    pub fn get_oldest_listen(&self) -> Option<&Arc<Listen>> {
        self.associated_listens
            .iter()
            .min_by_key(|listen| listen.listened_at)
    }
}
