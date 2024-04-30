use chrono::{DateTime, Utc};
use itertools::Itertools;
use listenbrainz::raw::{jspf::{Playlist, PlaylistInfo}, request::{PlaylistCreate, PlaylistCreatePlaylist, PlaylistCreatePlaylistExtension, PlaylistCreatePlaylistExtensionInner, PlaylistCreatePlaylistTrack}};

#[derive(Clone)]
pub struct PlaylistStub {
    title: String,
    username: Option<String>,
    public: bool,
    tracks: Vec<String>,
}

impl PlaylistStub {
    pub fn new(title: String, username: Option<String>, public: bool, tracks: Vec<String>) -> Self {
        Self { title, username, public, tracks }
    }
    
    pub fn into_jspf(self) -> PlaylistCreate {
        PlaylistCreate {
            playlist: PlaylistCreatePlaylist {
                title: self.title,
                track: self.tracks.iter().map(|id| {
                    PlaylistCreatePlaylistTrack {
                        identifier: format!("https://musicbrainz.org/recording/{}", id)
                    }
                }).collect(),
                extension: PlaylistCreatePlaylistExtension { 
                    musicbrainz: PlaylistCreatePlaylistExtensionInner { 
                        created_for: None, 
                        creator: self.username, 
                        collaborators: Vec::new(), 
                        copied_from: None, 
                        copied_from_deleted: None, 
                        public: self.public, 
                        last_modified_at: None, 
                        additional_metadata: None 
                    }
                }
            }
        }
    }
}