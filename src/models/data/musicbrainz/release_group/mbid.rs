use color_eyre::eyre::Context;
use derive_more::{Deref, DerefMut, Display, From, Into};
use musicbrainz_rs::entity::release_group::ReleaseGroup as ReleaseGroupMS;
use musicbrainz_rs::Fetch;
use serde::{Deserialize, Serialize};

use crate::core::entity_traits::mb_cached::MBCached;
use crate::core::entity_traits::mbid::IsMbid;
use crate::models::data::musicbrainz::external_musicbrainz_entity::ExternalMusicBrainzEntity;
use crate::models::data::musicbrainz::mbid::generic_mbid::{IdAliasState, MBIDSpe};
use crate::models::data::musicbrainz::mbid::is_musicbrainz_id::IsMusicbrainzID;
use crate::models::data::musicbrainz::mbid::MBID;
use crate::models::data::musicbrainz::release_group::external::ReleaseGroupExt;
use crate::models::data::musicbrainz::release_group::ReleaseGroup;
use crate::utils::println_mus;

#[derive(
    Debug, Clone, PartialEq, Eq, Deref, DerefMut, Into, From, Serialize, Deserialize, Hash, Display,
)]
pub struct ReleaseGroupMBID(String);

impl IsMbid<ReleaseGroup> for ReleaseGroupMBID {
    async fn get_or_fetch_entity(&self) -> color_eyre::Result<ReleaseGroup> {
        ReleaseGroup::get_cached_or_fetch(self).await
    }

    async fn fetch(&self) -> Result<ExternalMusicBrainzEntity, reqwest::Error> {
        println_mus(format!("Getting data for release group MBID: {}", &self));

        Ok(ReleaseGroupMS::fetch()
            .id(self)
            .with_artists()
            .with_annotations()
            .with_aliases()
            .with_genres()
            .with_ratings()
            .with_releases()
            //.with_release_group_relations() //FIXME: error decoding response body: no variant of enum RelationContent found in flattened data at line 1 column 751
            .with_series_relations()
            .with_url_relations()
            .with_tags()
            .execute()
            .await?
            .into_entity())
    }

    fn into_mbid(self) -> MBID {
        MBID::ReleaseGroup(self)
    }
}

impl<S> IsMusicbrainzID<ReleaseGroup> for MBIDSpe<ReleaseGroup, S>
where
    S: IdAliasState,
{
    async fn fetch(&self) -> color_eyre::Result<ExternalMusicBrainzEntity> {
        println_mus(format!("Getting data for release group MBID: {}", &self));

        Ok(ReleaseGroupMS::fetch()
            .id(self)
            .with_artists()
            .with_annotations()
            .with_aliases()
            .with_genres()
            .with_ratings()
            .with_releases()
            //.with_release_group_relations() //FIXME: error decoding response body: no variant of enum RelationContent found in flattened data at line 1 column 751
            .with_series_relations()
            .with_url_relations()
            .with_tags()
            .execute()
            .await
            .context("Failed to fetch release group from MusicBrainz")?
            .into_entity())
    }
}
