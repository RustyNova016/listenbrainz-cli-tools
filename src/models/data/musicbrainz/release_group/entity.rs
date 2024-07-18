use std::sync::Arc;

use crate::core::caching::musicbrainz::musicbrainz_cache::MusicbrainzCache;
use crate::models::data::musicbrainz::entity::any::any_musicbrainz_entity::AnyMusicBrainzEntity;
use crate::models::data::musicbrainz::entity::traits::MusicBrainzEntity;
use crate::models::data::musicbrainz::external_musicbrainz_entity::ExternalMusicBrainzEntity;
use crate::models::data::musicbrainz::mbid::state_id::state::PrimaryMBID;
use crate::models::data::musicbrainz::mbid::state_id::MBIDState;
use crate::models::data::musicbrainz::mbid::state_id::MBIDWithState;
use crate::models::data::musicbrainz_database::MUSICBRAINZ_DATABASE;
use crate::models::error::Error;
use crate::utils::println_mus;
use color_eyre::eyre::Context;
use musicbrainz_rs::entity::release_group::ReleaseGroup as ReleaseGroupMS;
use musicbrainz_rs::Fetch;

use super::external::ReleaseGroupExt;
use super::ReleaseGroup;

impl MusicBrainzEntity for ReleaseGroup {
    async fn fetch<S: MBIDState>(
        id: &MBIDWithState<Self, S>,
    ) -> color_eyre::Result<ExternalMusicBrainzEntity> {
        println_mus(format!("Getting data for release group MBID: {}", &id));

        Ok(ReleaseGroupMS::fetch()
            .id(id)
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

    fn get_cache() -> Arc<MusicbrainzCache<Self>> {
        MUSICBRAINZ_DATABASE.release_groups().clone()
    }

    fn get_mbid(&self) -> PrimaryMBID<Self> {
        PrimaryMBID::from(self.id.to_string().clone())
    }

    fn into_any(self: Arc<Self>) -> AnyMusicBrainzEntity {
        AnyMusicBrainzEntity::ReleaseGroup(self)
    }

    fn try_from_any(value: &AnyMusicBrainzEntity) -> Result<Arc<Self>, Error> {
        match value {
            AnyMusicBrainzEntity::ReleaseGroup(val) => Ok(val.clone()),
            _ => Err(Error::InvalidTypeConvertion(
                "AnyMusicBrainzEntity".to_string(),
                "ReleaseGroup".to_string(),
            )),
        }
    }

    fn incremental_update(self, newer: Self) -> Self {
        Self {
            id: newer.id,
            secondary_types: newer.secondary_types,
            secondary_type_ids: newer.secondary_type_ids,
            disambiguation: newer.disambiguation,
            title: newer.title,
            primary_type_id: newer.primary_type_id.or(self.primary_type_id),
            first_release_date: newer.first_release_date.or(self.first_release_date),
            primary_type: newer.primary_type.or(self.primary_type),
            tags: newer.tags.or(self.tags),
            aliases: newer.aliases.or(self.aliases),
            genres: newer.genres.or(self.genres),
            releases: newer.releases.or(self.releases),
            annotation: newer.annotation.or(self.annotation),
            artist_credit: newer.artist_credit.or(self.artist_credit),
            relations: newer.relations.or(self.relations),
        }
    }

    fn get_kind() -> crate::models::data::musicbrainz::entity::entity_kind::MusicbrainzEntityKind {
        crate::models::data::musicbrainz::entity::entity_kind::MusicbrainzEntityKind::ReleaseGroup
    }
}
