use derive_getters::Getters;
use itertools::Itertools;
use listenbrainz::raw::response::UserListensMBIDMapping;
use serde::{Deserialize, Serialize};

use crate::core::entity_traits::mb_cached::MBCached;
use crate::core::entity_traits::relations::has_artist_credits::HasArtistCredits;
use crate::models::data::musicbrainz::artist::mbid::ArtistMBID;
use crate::models::data::musicbrainz::recording::mbid::RecordingMBID;
use crate::models::data::musicbrainz::recording::Recording;
use crate::utils::extensions::UserListensMBIDMappingExt;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Getters)]
pub struct MappingData {
    /// The MBID of the recordings
    pub recording_mbid: String, // TODO: use Recording MBID

    /// Name of the recording
    pub recording_name: String,

    /// Artists MBID
    artist_mbid: Option<Vec<ArtistMBID>>,

    /// Artist credits:
    pub artist_credit: Option<String>,
}

impl MappingData {
    /// Get the mapped [`Recording`]
    pub async fn get_or_fetch_recording(&self) -> color_eyre::Result<Recording> {
        Recording::get_cache()
            .get_or_fetch(&self.recording_mbid.clone().into()) //TODO: Use MBID
            .await
    }

    /// Get the ids of the associated [`Recording`]'s [`Artist`]\(s)
    ///
    /// [`Artist`]: crate::models::data::musicbrainz::artist::Artist
    pub async fn get_or_fetch_artist_mbids(&self) -> color_eyre::Result<Vec<ArtistMBID>> {
        Ok(match &self.artist_mbid {
            None => self
                .get_or_fetch_recording()
                .await?
                .get_or_fetch_artist_credits()
                .await?
                .get_artist_ids(),
            Some(artists) => artists.clone(),
        })
    }

    pub fn get_recording_mbid(&self) -> RecordingMBID {
        self.recording_mbid.clone().into() // TODO: Use MBID
    }
}

impl From<UserListensMBIDMapping> for MappingData {
    fn from(value: UserListensMBIDMapping) -> Self {
        Self {
            recording_mbid: value.recording_mbid.clone(),
            recording_name: value
                .recording_name
                .clone()
                .unwrap_or_else(|| format!("Unknown Track ({})", value.recording_mbid)),
            artist_mbid: value
                .artist_mbids
                .clone()
                .map(|artists| artists.into_iter().map(Into::into).collect_vec()),
            artist_credit: value.get_artist_credit_as_string(),
        }
    }
}
