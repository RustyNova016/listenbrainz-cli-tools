use crate::models::cli::config::SelfEditActionValue;
use crate::models::cli::config::SelfEditType;
use crate::models::data::musicbrainz::mbid::MBID;
use derive_getters::Getters;
use derive_more::*;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize, Getters, Default, Clone)]
pub struct SelfEdit{
    on_radio_seeding: SelfEditAction,
    on_radio_insert: SelfEditAction,
    on_stat_counting: SelfEditAction
}

impl SelfEdit {
    pub fn set_action(&mut self, on: SelfEditType, action: SelfEditActionValue, edit_target: Option<MBID>) {
        match on {
            SelfEditType::RadioInsert => self.on_radio_insert = SelfEditAction::from_value(action, edit_target),
            SelfEditType::RadioSeeding => self.on_radio_seeding = SelfEditAction::from_value(action, edit_target),
            SelfEditType::StatCounting => self.on_stat_counting = SelfEditAction::from_value(action, edit_target),
        }
    }
}

#[derive(Debug, IsVariant, Unwrap, Serialize, Deserialize, Default, Clone)]
pub enum SelfEditAction {
    MergeInto(MBID),
    Abort,
    #[default]
    None
}

impl SelfEditAction {
    pub fn from_value(action: SelfEditActionValue, edit_target: Option<MBID>) -> Self {
        match action {
            SelfEditActionValue::Abort => Self::Abort,
            SelfEditActionValue::MergeInto => Self::MergeInto(edit_target.expect("Merge Into needs a target MBID")),
            SelfEditActionValue::None => Self::None
        }
    }
}