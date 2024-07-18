use std::sync::Arc;

use color_eyre::eyre::Error;
use derive_getters::Getters;
use futures::TryStream;
use futures::TryStreamExt;

use crate::models::data::musicbrainz::artist::Artist;
use crate::models::data::musicbrainz::artist_credit::ArtistCredit;
use crate::models::data::musicbrainz::entity::traits::MusicBrainzEntity;
use crate::models::data::musicbrainz::mbid::state_id::state::PrimaryMBID;
use crate::models::data::musicbrainz::recording::Recording;

use super::Listen;

pub mod stream;

#[derive(Clone, PartialEq, Eq, Debug, Getters)]
pub struct PrimaryListen {
    listen: Arc<Listen>,
    mapped_recording: Arc<Recording>,
}

impl PrimaryListen {
    pub async fn from_listen(listen: Arc<Listen>) -> Option<color_eyre::Result<Self>> {
        Some(
            listen
                .get_load_or_fetch_recording()
                .await?
                .map(|recording| Self {
                    mapped_recording: recording,
                    listen,
                }),
        )
    }

    pub fn get_mbid(&self) -> PrimaryMBID<Recording> {
        self.mapped_recording.get_mbid()
    }

    pub fn associate_credited_artist(
        self: Arc<Self>,
    ) -> impl TryStream<Ok = (Arc<Artist>, Arc<Self>), Error = color_eyre::Report> {
        self.mapped_recording()
            .artist_credit()
            .clone()
            .unwrap_or_default()
            .into_artist_stream()
            .map_ok(move |artist| (artist, self.clone()))
    }
}
