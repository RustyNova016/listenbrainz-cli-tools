//! This module contain all the prompts we could have about a recording.
//!
//! It's mostly full of "is_QUESTION(&self) -> bool" methods

use tokio::stream;

use crate::models::data::musicbrainz::relation::has_relationships::HasRelationships;
use crate::models::data::musicbrainz::relation::inspections::VecRelationExt;
use crate::models::data::musicbrainz::relation::RelationTarget;

use super::Recording;

impl Recording {
    pub async fn is_streamable_on(&self, domain: &str) -> color_eyre::Result<bool> {
        if let Some(rels) = self.relations {
            if rels.is_streamable_on(domain).await? {
                return Ok(true);
            }
        }

        for release in self.get_or_fetch_releases().await? {
            if let Some(rels) = release.get_relationships() {
                if rels.is_streamable_on(domain).await? {
                    return Ok(true);
                }
            }
        }

        Ok(false)
    }
}
