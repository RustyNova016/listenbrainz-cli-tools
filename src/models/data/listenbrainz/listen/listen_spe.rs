use std::collections::HashMap;

use chrono::{DateTime, Utc};
use color_eyre::eyre::Context;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};

use crate::models::data::listenbrainz::mapping_data::MappingData;
use crate::models::data::listenbrainz::messybrainz::MessyBrainzData;
use crate::models::data::musicbrainz::mbid::generic_mbid::MBIDSpe;
use crate::models::data::musicbrainz::mbid::generic_mbid::PrimaryID;
use crate::models::data::musicbrainz::recording::Recording;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Getters)]
pub struct ListenSpe<S>
where
    S: MappingState,
{
    /// The username of the user who listened to it
    pub(super) user: String,

    /// Time of when the listen happened
    pub(super) listened_at: DateTime<Utc>,

    /// Data that have been sent to listenbrainz durring listen submition
    pub(super) messybrainz_data: MessyBrainzData,

    /// Data of the mapping
    pub(super) mapping_data: S,
}

// Typestate
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Getters)]
pub struct Unmapped {}
pub type MappedNaive = MappingData;
pub type MappedPrimary = MBIDSpe<Recording, PrimaryID>;

pub trait MappingState {}
impl MappingState for Unmapped {}
impl MappingState for MappedNaive {}
impl MappingState for MappedPrimary {}

// Base impls
impl<S> ListenSpe<S>
where
    S: MappingState,
{
    pub fn get_listened_at(&self) -> &DateTime<Utc> {
        &self.listened_at
    }

    pub fn get_messybrain_data(&self) -> &MessyBrainzData {
        &self.messybrainz_data
    }

    /// Send a mapping request to Listenbrainz
    pub async fn submit_mapping(&self, mbid: &str, token: &str) -> color_eyre::Result<()> {
        let client = reqwest::Client::new();

        let mut body_json = HashMap::new();
        body_json.insert("recording_msid", self.get_messybrain_data().msid.clone());
        body_json.insert("recording_mbid", mbid.to_owned());

        client
            .post("https://api.listenbrainz.org/1/metadata/submit_manual_mapping/")
            .header("Authorization", format!("Token {}", token.to_owned()))
            .json(&body_json)
            .send()
            .await
            .context("Couldn't send the mapping to Listenbrainz")?
            .error_for_status()
            .context("Listenbrainz returned an error")?;

        Ok(())
    }
}
