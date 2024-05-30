use crate::models::data::musicbrainz::recording::mbid::RecordingMBID;
use derive_builder::Builder;
use listenbrainz::raw::request::{
    PlaylistCreate, PlaylistCreatePlaylist, PlaylistCreatePlaylistExtension,
    PlaylistCreatePlaylistExtensionInner, PlaylistCreatePlaylistTrack,
};

#[derive(Clone, Builder)]
pub struct PlaylistStub {
    title: String,
    #[allow(dead_code)] // Temporary fix
    description: Option<String>,
    username: Option<String>,
    public: bool,
    tracks: Vec<RecordingMBID>,
}

impl PlaylistStub {
    pub fn new(
        title: String,
        username: Option<String>,
        public: bool,
        tracks: Vec<RecordingMBID>,
        description: Option<String>,
    ) -> Self {
        Self {
            title,
            username,
            public,
            tracks,
            description,
        }
    }

    pub fn into_jspf(self) -> PlaylistCreate {
        PlaylistCreate {
            playlist: PlaylistCreatePlaylist {
                title: self.title,
                track: self
                    .tracks
                    .iter()
                    .map(|id| PlaylistCreatePlaylistTrack {
                        identifier: format!("https://musicbrainz.org/recording/{id}"),
                    })
                    .collect(),
                extension: PlaylistCreatePlaylistExtension {
                    musicbrainz: PlaylistCreatePlaylistExtensionInner {
                        created_for: None,
                        creator: self.username,
                        collaborators: Vec::new(),
                        copied_from: None,
                        copied_from_deleted: None,
                        public: self.public,
                        last_modified_at: None,
                        additional_metadata: None,
                    },
                },
                annotation: self.description,
            },
        }
    }
}
