use crate::core::entity_traits::mbid::IsMbid;
use crate::models::cli::config::SelfEditContext;
use crate::models::config::self_edits::SelfEditAction;
use crate::models::config::Config;
use crate::models::data::musicbrainz::recording::mbid::RecordingMBID;

use super::ListenCollection;

impl ListenCollection {
    pub fn apply_configuration(&self, config: &Config, context: &SelfEditContext) -> Self {
        let mut results = Vec::new();

        for listen in self.iter() {
            let Some(recording_string) = listen.get_recording_mbid_as_string() else {
                results.push(listen.clone());
                continue;
            };

            let recording: RecordingMBID = recording_string.clone().into();

            let Some(edit) = config.self_edits().get(&recording.clone().into_mbid()) else {
                results.push(listen.clone());
                continue;
            };

            let action = edit.get_action(context);

            match action {
                SelfEditAction::Abort => {}
                SelfEditAction::MergeInto(val) => {
                    listen
                        .as_ref()
                        .clone()
                        .set_recording_mapping(val.clone().unwrap_recording());
                    results.push(listen.clone());
                }
                SelfEditAction::None => {
                    results.push(listen.clone());
                }
            }
        }

        Self { data: results }
    }
}
