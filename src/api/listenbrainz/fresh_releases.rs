use chrono::DateTime;
use chrono::Utc;
use macon::Builder;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Builder)]
pub struct FreshReleaseRequest {
    #[builder(Default=!)]
    release_date: DateTime<Utc>,
    days: u8,
    past: bool,
    future: bool,
}

impl FreshReleaseRequest {
    pub fn get_parameters(&self) -> String {
        format!(
            "?release_date={}&days={}&past={}&future={}",
            self.release_date.format("%Y-%m-%d"),
            self.days,
            self.past,
            self.future
        )
    }

    pub async fn fetch(&self) -> Result<Vec<FreshReleaseResponse>, crate::Error> {
        let response = reqwest::get(format!(
            "https://api.listenbrainz.org/1/explore/fresh-releases/{}",
            self.get_parameters()
        ))
        .await?;

        println!("{:#?}", response.text().await.unwrap());

        let response = reqwest::get(format!(
            "https://api.listenbrainz.org/1/explore/fresh-releases/{}",
            self.get_parameters()
        ))
        .await?;

        Ok(response.json().await?)
    }
}

impl Default for FreshReleaseRequest {
    fn default() -> Self {
        Self {
            days: 90,
            future: true,
            past: true,

            release_date: Default::default(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FreshReleaseResponse {
    pub artist_credit_name: String,
    pub artist_mbids: Vec<String>,

    pub release_date: String,
    pub release_group_mbid: String,
    pub release_group_primary_type: String,
    pub release_mbid: String,
    pub release_name: String,
}
