use std::fmt::Display;

use color_eyre::eyre::Ok;
use regex::Regex;

use crate::core::entity_traits::mbid::IsMbid;
use crate::models::data::musicbrainz::mbid::MBID;
use crate::models::data::musicbrainz::recording::mbid::RecordingMBID;
use crate::models::data::musicbrainz::recording::Recording;
use crate::models::data::musicbrainz::relation::collection::VecRelationExt;
use crate::models::data::musicbrainz::relation::type_ids::PERFORMANCE_RELATIONSHIP;

use super::IsClippyError;

pub struct MissingWorkError {
    recording_id: RecordingMBID,
    recording: Recording,
    recording_string: String,
}

impl IsClippyError for MissingWorkError {
    async fn check_for_error(id: MBID) -> color_eyre::Result<Option<Self>> {
        let MBID::Recording(id) = id else {
            return Ok(None);
        };
        let recording = id.get_or_fetch_entity().await?;

        if recording.relations().as_ref().is_some_and(|rels| {
            rels.find_relation_type_id(PERFORMANCE_RELATIONSHIP)
                .is_some()
        }) {
            return Ok(None);
        }

        let recording_string = recording.get_title_with_credits().await?;
        Ok(Some(Self {
            recording_id: id,
            recording,
            recording_string,
        }))
    }

    fn get_title(&self) -> String {
        format!("Missing work for recording: `{}`", self.recording_string)
    }

    fn get_relevant_url(&self) -> String {
        self.recording_id.get_link()
    }

    fn get_description(&self) -> String {
        let description = "All recordings should have works associated to it. It is quite useful to track references and similar but different recordings.\n
        You can associate a work with a \"recording of\" (Recording - Work) relationship".to_string().replace("\n\n.*", "\n");

        let regex = Regex::new(r"(?m)\n\n\s*").unwrap();
        let substitution = "\n";
        regex.replace_all(&description, substitution).to_string()
    }

    fn get_additions(&self) -> Vec<(String, String)> {
        let additions = Vec::new();

        additions
    }
}
