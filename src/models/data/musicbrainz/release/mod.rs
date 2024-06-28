use std::sync::Arc;

use chrono::NaiveDate;
use derive_getters::Getters;
use musicbrainz_rs::entity::alias::Alias;
use musicbrainz_rs::entity::genre::Genre;
use musicbrainz_rs::entity::release::{ReleasePackaging, ReleaseStatus};
use musicbrainz_rs::entity::tag::Tag;
use serde::{Deserialize, Serialize};

use crate::core::caching::musicbrainz::musicbrainz_cache::MusicbrainzCache;
use crate::core::entity_traits::relations::has_artist_credits::HasArtistCredits;
use crate::core::entity_traits::relations::has_release_group::HasReleaseGroup;
use crate::models::data::musicbrainz::artist_credit::collection::ArtistCredits;
use crate::models::data::musicbrainz::entity::entity_kind::MusicbrainzEntityKind;
use crate::models::data::musicbrainz::entity::is_musicbrainz_entity::IsMusicbrainzEntity;
use crate::models::data::musicbrainz::mbid::generic_mbid::{MBIDSpe, PrimaryID};
use crate::models::data::musicbrainz::relation::Relation;
use crate::models::data::musicbrainz::release_group::mbid::ReleaseGroupMBID;
use crate::models::data::musicbrainz_database::MUSICBRAINZ_DATABASE;

use self::mbid::ReleaseMBID;
use self::media::Media;

pub mod external;

pub mod caching;
pub mod converters;
pub mod get_or_fetch;
pub mod getters;
pub mod mbid;
pub mod media;
pub mod track;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize, Getters)]
pub struct Release {
    id: ReleaseMBID,
    title: String,
    status_id: Option<String>,
    status: Option<ReleaseStatus>,
    date: Option<NaiveDate>,
    country: Option<String>,
    //quality: Option<ReleaseQuality>, //TODO: Mirror renaming #[serde(rename_all(deserialize = "lowercase"))]
    barcode: Option<String>,
    disambiguation: Option<String>,
    packaging_id: Option<String>,
    packaging: Option<ReleasePackaging>,
    relations: Option<Vec<Relation>>,
    release_group: Option<ReleaseGroupMBID>,
    artist_credit: Option<ArtistCredits>,
    media: Option<Vec<Media>>,
    //label_info: Option<Vec<LabelInfo>>,
    tags: Option<Vec<Tag>>,
    aliases: Option<Vec<Alias>>,
    genres: Option<Vec<Genre>>,
    annotation: Option<String>,
}

impl IsMusicbrainzEntity for Release {
    // fn get_mb_cache() -> Arc<MusicbrainzCache<Self>> {
    //     MUSICBRAINZ_DATABASE.releases().clone()
    // }
    fn as_kind(&self) -> MusicbrainzEntityKind {
        MusicbrainzEntityKind::Release
    }

    fn get_mbidspe(&self) -> MBIDSpe<Self, PrimaryID> {
        MBIDSpe::from(self.id.to_string())
    }

    fn partial_update(self, newer: Self) -> Self {
        Self {
            annotation: newer.annotation.or(self.annotation),
            barcode: newer.barcode.or(self.barcode),
            country: newer.country.or(self.country),
            disambiguation: newer.disambiguation.or(self.disambiguation),
            media: newer.media.or(self.media),
            packaging_id: newer.packaging_id.or(self.packaging_id),
            status_id: newer.status_id.or(self.status_id),
            title: newer.title,
            id: newer.id,
            artist_credit: newer.artist_credit.or(self.artist_credit),
            release_group: newer.release_group.or(self.release_group),
            relations: newer.relations.or(self.relations),
            aliases: newer.aliases.or(self.aliases),
            date: newer.date.or(self.date),
            genres: newer.genres.or(self.genres),
            packaging: newer.packaging.or(self.packaging),
            //quality: newer.quality.or(self.quality),
            status: newer.status.or(self.status),
            tags: newer.tags.or(self.tags),
        }
    }

    fn into_any(self: Arc<Self>) -> super::entity::any_musicbrainz_entity::AnyMusicBrainzEntity {
        self.into()
    }
}

impl HasArtistCredits<ReleaseMBID> for Release {
    fn get_artist_credits(&self) -> &Option<ArtistCredits> {
        &self.artist_credit
    }
}

impl HasReleaseGroup<ReleaseMBID> for Release {
    fn get_release_group(&self) -> &Option<ReleaseGroupMBID> {
        &self.release_group
    }
}
