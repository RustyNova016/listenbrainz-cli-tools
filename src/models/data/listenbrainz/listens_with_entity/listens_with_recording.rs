use std::sync::Arc;

use rust_decimal::Decimal;

use crate::models::data::listenbrainz::listen::collection::mapped_primary_collection::MappedPrimaryListenCollectionExt;
use crate::models::data::listenbrainz::listen::collection::mapped_primary_collection::PrimaryListenCollection;
use crate::models::data::musicbrainz::entity::traits::MusicBrainzEntity;
use crate::models::data::musicbrainz::recording::Recording;

use super::ListensWithEntity;

impl ListensWithEntity<Recording> {
    /// Create a new [`ListenWithEntity`] from a vector of listens. It keep only direct listens
    pub fn from_unfiltered(entity: Arc<Recording>, listens: &PrimaryListenCollection) -> Self {
        let filtered = listens.where_mapped_recording_eq(&entity.get_mbid());

        Self {
            entity,
            listens: filtered,
        }
    }

    pub async fn underated_score_single(&self) -> color_eyre::Result<Decimal> {
        Ok(self
            .listens
            .clone()
            .into_legacy()
            .get_underrated_recordings()
            .await?
            .first()
            .expect("Recording should have a score")
            .0)
    }
}
