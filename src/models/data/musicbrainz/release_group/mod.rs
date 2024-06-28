use std::sync::Arc;

use chrono::NaiveDate;
use derive_getters::Getters;
use musicbrainz_rs::entity::alias::Alias;
use musicbrainz_rs::entity::genre::Genre;
use musicbrainz_rs::entity::release_group::{ReleaseGroupPrimaryType, ReleaseGroupSecondaryType};
use musicbrainz_rs::entity::tag::Tag;
use serde::{Deserialize, Serialize};

use crate::core::entity_traits::relations::has_artist_credits::HasArtistCredits;
use crate::models::data::musicbrainz::artist_credit::collection::ArtistCredits;
use crate::models::data::musicbrainz::entity::entity_kind::MusicbrainzEntityKind;
use crate::models::data::musicbrainz::entity::is_musicbrainz_entity::IsMusicbrainzEntity;
use crate::models::data::musicbrainz::mbid::generic_mbid::{MBIDSpe, PrimaryID};
use crate::models::data::musicbrainz::relation::Relation;
use crate::models::data::musicbrainz::release::mbid::ReleaseMBID;
use crate::models::data::musicbrainz::release_group::mbid::ReleaseGroupMBID;

mod caching;
mod converters;
pub(crate) mod external;
pub mod mbid;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize, Getters)]
pub struct ReleaseGroup {
    id: ReleaseGroupMBID,
    primary_type_id: Option<String>,
    primary_type: Option<ReleaseGroupPrimaryType>,
    secondary_type_ids: Vec<String>,
    secondary_types: Vec<ReleaseGroupSecondaryType>,
    first_release_date: Option<NaiveDate>,
    title: String,
    disambiguation: String,
    relations: Option<Vec<Relation>>,
    artist_credit: Option<ArtistCredits>,
    releases: Option<Vec<ReleaseMBID>>,
    tags: Option<Vec<Tag>>,
    //rating: Option<Rating>,
    aliases: Option<Vec<Alias>>,
    genres: Option<Vec<Genre>>,
    annotation: Option<String>,
}

impl IsMusicbrainzEntity for ReleaseGroup {
    // fn get_mb_cache() -> Arc<MusicbrainzCache<Self>> {
    //     MUSICBRAINZ_DATABASE.release_groups().clone()
    // }

    fn as_kind(&self) -> MusicbrainzEntityKind {
        MusicbrainzEntityKind::ReleaseGroup
    }

    fn get_mbidspe(&self) -> MBIDSpe<Self, PrimaryID> {
        MBIDSpe::from(self.id.to_string())
    }

    fn partial_update(self, newer: Self) -> Self {
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

    fn into_any(self: Arc<Self>) -> super::entity::any_musicbrainz_entity::AnyMusicBrainzEntity {
        self.into()
    }

    fn into_arc_and_any(self) -> super::entity::any_musicbrainz_entity::AnyMusicBrainzEntity {
        Arc::new(self).into_any()
    }
}

impl HasArtistCredits<ReleaseGroupMBID> for ReleaseGroup {
    fn get_artist_credits(&self) -> &Option<ArtistCredits> {
        &self.artist_credit
    }
}
