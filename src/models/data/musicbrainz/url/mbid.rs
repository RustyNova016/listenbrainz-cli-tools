use crate::core::entity_traits::mbid::IsMbid;
use crate::models::data::musicbrainz::external_musicbrainz_entity::ExternalMusicBrainzEntity;
use crate::models::data::musicbrainz::mbid::MBID;
use crate::models::data::musicbrainz::url::URL;
use crate::models::data::musicbrainz::work::Work;
use crate::utils::println_mus;
use color_eyre::eyre::Context;
use derive_more::{Deref, DerefMut, Display, From, Into};
use musicbrainz_rs::entity::url::Url;
use musicbrainz_rs::Fetch;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, PartialEq, Eq, Deref, DerefMut, Into, From, Serialize, Deserialize, Hash, Display,
)]
pub struct URLMBID(String);

impl IsMbid<URL> for URLMBID {
    async fn get_or_fetch_entity(&self) -> color_eyre::Result<Work> {
        URL::get_cache().get_or_fetch(self).await
    }

    async fn fetch(&self) -> color_eyre::Result<ExternalMusicBrainzEntity> {
        println_mus(format!("Getting data for work MBID: {}", &self));

        Ok(Url::fetch()
            .id(self)
            .with_url_relations()
            .with_artist_relations()
            .execute()
            .await
            .context("Failed to fetch url from MusicBrainz")?
            .into_entity())
    }

    fn into_mbid(self) -> MBID {
        MBID::Url(self)
    }

    fn get_link(&self) -> String {
        format!("https://musicbrainz.org/url/{self}")
    }
}
