use color_eyre::eyre::Context;
use derive_more::{Deref, DerefMut, Display, From, Into};
use musicbrainz_rs::entity::work::Work as WorkMS;
use musicbrainz_rs::Fetch;
use serde::Deserialize;
use serde::Serialize;

use crate::core::entity_traits::mb_cached::MBCached;
use crate::core::entity_traits::mbid::IsMbid;
use crate::models::data::musicbrainz::external_musicbrainz_entity::ExternalMusicBrainzEntity;
use crate::models::data::musicbrainz::mbid::generic_mbid::{IdAliasState, MBIDSpe};
use crate::models::data::musicbrainz::mbid::is_musicbrainz_id::IsMusicbrainzID;
use crate::models::data::musicbrainz::mbid::MBID;
use crate::models::data::musicbrainz::work::external::WorkExt;
use crate::utils::println_mus;

use super::Work;

#[derive(
    Debug, Clone, PartialEq, Eq, Deref, DerefMut, Into, From, Serialize, Deserialize, Hash, Display,
)]
pub struct WorkMBID(String);

impl IsMbid<Work> for WorkMBID {
    async fn get_or_fetch_entity(&self) -> color_eyre::Result<Work> {
        Work::get_cache().get_or_fetch(self).await
    }

    async fn fetch(&self) -> color_eyre::Result<ExternalMusicBrainzEntity> {
        println_mus(format!("Getting data for work MBID: {}", &self));

        Ok(WorkMS::fetch()
            .id(self)
            .with_aliases()
            .with_annotations()
            .with_artist_relations()
            .with_genres()
            .with_label_relations()
            .with_ratings()
            .with_recording_relations()
            .with_tags()
            .with_url_relations()
            .with_work_relations()
            .execute()
            .await
            .context("Failed to fetch work from MusicBrainz")?
            .into_entity())
    }

    fn into_mbid(self) -> MBID {
        MBID::Work(self)
    }
}

impl<S> IsMusicbrainzID<Work> for MBIDSpe<Work, S>
where
    S: IdAliasState,
{
    async fn fetch(&self) -> color_eyre::Result<ExternalMusicBrainzEntity> {
        println_mus(format!("Getting data for work MBID: {}", &self));

        Ok(WorkMS::fetch()
            .id(self)
            .with_aliases()
            .with_annotations()
            .with_artist_relations()
            .with_genres()
            .with_label_relations()
            .with_ratings()
            .with_recording_relations()
            .with_tags()
            .with_url_relations()
            .with_work_relations()
            .execute()
            .await
            .context("Failed to fetch work from MusicBrainz")?
            .into_entity())
    }
}
