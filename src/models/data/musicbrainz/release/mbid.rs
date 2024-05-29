use color_eyre::eyre::Context;
use derive_more::{Deref, DerefMut, Display, From, Into};
use musicbrainz_rs::entity::release::Release as ReleaseMS;
use musicbrainz_rs::Fetch;
use serde::{Deserialize, Serialize};

use crate::core::entity_traits::mb_cached::MBCached;
use crate::core::entity_traits::mbid::IsMbid;
use crate::models::data::musicbrainz::external_musicbrainz_entity::ExternalMusicBrainzEntity;
use crate::models::data::musicbrainz::mbid::MBID;
use crate::models::data::musicbrainz::release::external::ReleaseExt;
use crate::models::data::musicbrainz::release::Release;
use crate::utils::println_mus;

#[derive(Debug, Clone, PartialEq, Eq, Deref, DerefMut, Into, From, Serialize, Deserialize, Display, Hash)]
pub struct ReleaseMBID(String);

impl IsMbid<Release> for ReleaseMBID {
    async fn get_or_fetch_entity(&self) -> color_eyre::Result<Release> {
        Release::get_cached_or_fetch(self).await
    }

    async fn fetch(&self) -> color_eyre::Result<ExternalMusicBrainzEntity> {
        println_mus(format!("Getting data for release MBID: {}", &self));

        Ok(ReleaseMS::fetch()
            .id(self)
            .with_artists()
            .with_artist_credits()
            .with_release_groups()
            .with_artist_credits()
            .with_aliases()
            .with_annotations()
            .with_artist_relations()
            .with_recordings()
            .with_recording_level_relations()
            //.with_work_level_relations() https://github.com/oknozor/musicbrainz_rs/pull/87
            //.with_work_relations()
            .execute()
            .await
            .context("Failed to fetch release from MusicBrainz")?
            .into_entity())
    }

    fn into_mbid(self) -> MBID {
        MBID::Release(self)
    }
}
