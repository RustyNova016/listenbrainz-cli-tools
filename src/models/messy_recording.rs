use listenbrainz::raw::response::UserListensListen;

/// Represent a messybrain recording id
pub struct MessyRecording {
    pub id: String,
    pub associated_listens: Vec<UserListensListen>,
}

impl MessyRecording {
    pub fn new(id: String) -> Self {
        MessyRecording {
            id,
            associated_listens: Vec::new(),
        }
    }

    pub fn add_listen(&mut self, listen: UserListensListen) {
        if listen.recording_msid == self.id {
            self.associated_listens.push(listen)
        }
    }

    pub fn get_recording_name(&self) -> Option<String> {
        self.associated_listens
            .iter()
            .find(|listen| !listen.track_metadata.track_name.is_empty())
            .map(|listen| listen.track_metadata.track_name.clone())
    }

    pub fn get_artist_name(&self) -> Option<String> {
        self.associated_listens
            .iter()
            .find(|listen| !listen.track_metadata.artist_name.is_empty())
            .map(|listen| listen.track_metadata.artist_name.clone())
    }

    pub fn get_latest_listen(&self) -> Option<&UserListensListen> {
        self.associated_listens
            .iter()
            .max_by_key(|listen| listen.listened_at)
    }
}
