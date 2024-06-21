use std::collections::HashMap;

use color_eyre::eyre::Context;

use crate::models::data::musicbrainz::recording::mbid::RecordingMBID;

pub struct MSID(String);

impl MSID {
    pub async fn submit_mapping(&self, mbid: RecordingMBID, token: &str) -> color_eyre::Result<()> {
        let client = reqwest::Client::new();

        let mut body_json = HashMap::new();
        body_json.insert("recording_msid", self.0.clone());
        body_json.insert("recording_mbid", mbid.to_string());

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

impl From<String> for MSID {
    fn from(value: String) -> Self {
        Self(value)
    }
}
