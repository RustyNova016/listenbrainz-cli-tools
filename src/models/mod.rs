use listenbrainz::raw::response::UserListensListen;

pub mod api;
pub mod cache;
pub mod cli;
pub mod data;
pub mod messy_recording;
pub mod musicbrainz;
pub mod stats;

pub struct UnlinkedListenCollection(Vec<UserListensListen>);

impl UnlinkedListenCollection {
    pub fn push(&mut self, item: UserListensListen) {
        if item.track_metadata.mbid_mapping.is_none() {
            self.0.push(item)
        }
    }
}

impl Extend<UserListensListen> for UnlinkedListenCollection {
    fn extend<T: IntoIterator<Item = UserListensListen>>(&mut self, iter: T) {
        self.0.extend(iter)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Verbosity {
    None,
    Info,
}

impl Verbosity {
    /// Return true if the verbosity level is at least "info"
    fn at_least_info(&self) -> bool {
        match self {
            Verbosity::None => false,
            _ => true,
        }
    }
}
