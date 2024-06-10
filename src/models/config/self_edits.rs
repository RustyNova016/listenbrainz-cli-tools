use std::collections::HashMap;
use std::hash::Hash;

use color_eyre::owo_colors::OwoColorize;
use derive_getters::Getters;
use derive_more::*;
use itertools::Itertools;
use serde::Deserialize;
use serde::Serialize;

use crate::models::cli::config::SelfEditActionValue;
use crate::models::cli::config::SelfEditContext;
use crate::models::data::musicbrainz::mbid::MBID;
use crate::utils::println_cli;

#[derive(Debug, Serialize, Deserialize, Getters, Default, Clone)]
pub struct SelfEdit {
    on_radio_seeding: SelfEditAction,
    on_radio_insert: SelfEditAction,
    on_stat_counting: SelfEditAction,
}

impl SelfEdit {
    pub fn set_action(
        &mut self,
        on: SelfEditContext,
        action: SelfEditActionValue,
        edit_target: Option<MBID>,
    ) {
        match on {
            SelfEditContext::RadioInsert => {
                self.on_radio_insert = SelfEditAction::from_value(action, edit_target);
            }
            SelfEditContext::RadioSeeding => {
                self.on_radio_seeding = SelfEditAction::from_value(action, edit_target);
            }
            SelfEditContext::StatCounting => {
                self.on_stat_counting = SelfEditAction::from_value(action, edit_target);
            }
        }
    }

    pub fn get_action(&self, context: &SelfEditContext) -> &SelfEditAction {
        match context {
            SelfEditContext::RadioInsert => &self.on_radio_insert,
            SelfEditContext::RadioSeeding => &self.on_radio_seeding,
            SelfEditContext::StatCounting => &self.on_stat_counting,
        }
    }
}

#[derive(Debug, IsVariant, Unwrap, Serialize, Deserialize, Default, Clone)]
pub enum SelfEditAction {
    MergeInto(MBID),
    Abort,
    #[default]
    None,
}

impl SelfEditAction {
    pub fn from_value(action: SelfEditActionValue, edit_target: Option<MBID>) -> Self {
        match action {
            SelfEditActionValue::Abort => Self::Abort,
            SelfEditActionValue::MergeInto => {
                Self::MergeInto(edit_target.expect("Merge Into needs a target MBID"))
            }
            SelfEditActionValue::None => Self::None,
        }
    }
}

#[derive(Debug, Clone, Deref, DerefMut, Serialize, Deserialize, Default)]
#[serde(from = "Vec<(MBID, SelfEdit)>")]
#[serde(into = "Vec<(MBID, SelfEdit)>")]
pub struct EditMap {
    data: HashMap<MBID, SelfEdit>,
}

impl EditMap {
    pub fn into_hashmap(self) -> HashMap<MBID, SelfEdit> {
        self.data
    }

    pub fn apply_edit_map(&self, ids: Vec<MBID>, context: SelfEditContext) -> Vec<MBID> {
        let mut results = Vec::new();

        for id in ids.into_iter() {
            let Some(edit) = self.get(&id) else {
                results.push(id);
                continue;
            };

            match edit.get_action(&context) {
                SelfEditAction::None => results.push(id),
                SelfEditAction::MergeInto(val) => results.push(val.clone()),
                SelfEditAction::Abort => {}
            }
        }

        results
    }

    pub fn apply_action_for_context(&self, id: MBID, context: &SelfEditContext) -> Option<MBID> {
        let Some(edit) = self.get(&id) else {
            return Some(id);
        };

        match edit.get_action(&context) {
            SelfEditAction::None => Some(id),
            SelfEditAction::MergeInto(val) => {
                #[cfg(debug_assertions)]
                println_cli(format!("Merging {id} into {val}").blue());
                Some(val.clone())
            }
            SelfEditAction::Abort => {
                #[cfg(debug_assertions)]
                println_cli(format!("Ignoring {id}'s existence").blue());
                None
            }
        }
    }
}

impl From<Vec<(MBID, SelfEdit)>> for EditMap {
    fn from(value: Vec<(MBID, SelfEdit)>) -> Self {
        let mut data = HashMap::new();

        for (key, val) in value.into_iter() {
            data.insert(key, val);
        }

        Self { data }
    }
}

impl From<EditMap> for Vec<(MBID, SelfEdit)> {
    fn from(value: EditMap) -> Self {
        value.into_hashmap().into_iter().collect_vec()
    }
}
