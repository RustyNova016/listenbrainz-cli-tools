use itertools::Itertools;
use crate::core::entity_traits::mbid::{IsMbid, VecIExt};
use crate::models::clippy::clippy_error::IsClippyError;
use crate::models::data::musicbrainz::mbid::MBID;
use crate::models::data::musicbrainz::relation::has_relationships::{HasRelationships, VecTExt};
use crate::models::data::musicbrainz::relation::type_ids::FREE_STREAMING;
use crate::models::data::musicbrainz::release::Release;

pub struct MissingISRC {
    release: Release
}

impl IsClippyError for MissingISRC {
    async fn check_for_error(id: MBID) -> color_eyre::Result<Option<Self>> {
        // Check if it is a recording
        let MBID::Recording(id) = id else {return Ok(None)};

        let recording = id.get_or_fetch_entity().await?;
        // Check if it has ISRCs
        if recording.isrcs().is_some_and(|data| !data.is_empty()) {return Ok(None)}

        // Check if there is a release with a Spotify URL
        let releases = recording.get_or_fetch_releases().await?;
        let relations = releases.into_all_relationships();
        let url_relations = relations.into_iter().filter(|rel| rel.content().is_url()).collect_vec();

        if url_relations.iter().any(|relation| relation.type_id() == FREE_STREAMING && relation.content().unwrap_url() ==) {

        }
    }

    fn get_title(&self) -> String {
        todo!()
    }

    fn get_relevant_url(&self) -> String {
        todo!()
    }

    fn get_description(&self) -> String {
        todo!()
    }

    fn get_additions(&self) -> Vec<(String, String)> {
        todo!()
    }
}