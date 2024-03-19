use listenbrainz::raw::response::UserListensListen;

pub mod cli;
pub mod data;
pub mod messy_recording;
pub mod musicbrainz;
pub mod stats;
//pub mod api;

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
