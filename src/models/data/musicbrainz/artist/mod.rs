use std::sync::Arc;

use derive_getters::Getters;
use itertools::Itertools;
use musicbrainz_rs::entity::alias::Alias;
use musicbrainz_rs::entity::artist::{ArtistType, Gender};
use musicbrainz_rs::entity::genre::Genre;
use musicbrainz_rs::entity::lifespan::LifeSpan;
use musicbrainz_rs::entity::tag::Tag;
use serde::{Deserialize, Serialize};

use crate::core::caching::musicbrainz::musicbrainz_cache::MusicbrainzCache;
use crate::models::data::musicbrainz::mbid::generic_mbid::{MBIDSpe, PrimaryID};
use crate::models::data::musicbrainz::relation::Relation;
use crate::models::data::musicbrainz::work::mbid::WorkMBID;
use crate::models::data::musicbrainz_database::MUSICBRAINZ_DATABASE;
use crate::models::error::Error;

use super::entity::any::any_musicbrainz_entity::AnyMusicBrainzEntity;
use super::entity::entity_kind::MusicbrainzEntityKind;
use super::entity::is_musicbrainz_entity::IsMusicbrainzEntity;
use super::recording::mbid::RecordingMBID;
use super::release::mbid::ReleaseMBID;
use super::release_group::mbid::ReleaseGroupMBID;

use self::mbid::ArtistMBID;

pub mod caching;
pub mod external;
pub mod getters;
pub mod mbid;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize, Getters)]
#[serde(rename_all = "kebab-case")]
pub struct Artist {
    id: ArtistMBID,
    name: String,
    sort_name: String,
    disambiguation: String,
    artist_type: Option<ArtistType>,
    gender: Option<Gender>,
    //pub area: Option<Area>,
    //pub begin_area: Option<Area>,
    relations: Option<Vec<Relation>>,
    releases: Option<Vec<ReleaseMBID>>,
    works: Option<Vec<WorkMBID>>,
    release_groups: Option<Vec<ReleaseGroupMBID>>,
    recordings: Option<Vec<RecordingMBID>>,
    aliases: Option<Vec<Alias>>,
    tags: Option<Vec<Tag>>,
    genres: Option<Vec<Genre>>,
    //pub rating: Option<Rating>,
    country: Option<String>,
    annotation: Option<String>,
    life_span: Option<LifeSpan>,
}

impl From<musicbrainz_rs::entity::artist::Artist> for Artist {
    fn from(artist: musicbrainz_rs::entity::artist::Artist) -> Self {
        Self {
            id: artist.id.into(),
            name: artist.name,
            aliases: artist.aliases,
            annotation: artist.annotation,
            artist_type: artist.artist_type,
            country: artist.country,
            disambiguation: artist.disambiguation,
            gender: artist.gender,
            genres: artist.genres,
            life_span: artist.life_span,
            recordings: None,
            release_groups: None,
            releases: None,
            sort_name: artist.sort_name,
            tags: artist.tags,
            works: None,
            relations: artist
                .relations
                .map(|relations| relations.into_iter().map_into().collect_vec()),
        }
    }
}

impl IsMusicbrainzEntity for Artist {
    fn get_mb_cache() -> Arc<MusicbrainzCache<Self>> {
        MUSICBRAINZ_DATABASE.artists().clone()
    }

    fn as_kind(&self) -> MusicbrainzEntityKind {
        MusicbrainzEntityKind::Artist
    }

    fn get_mbidspe(&self) -> MBIDSpe<Self, PrimaryID> {
        MBIDSpe::from(self.id.to_string())
    }

    fn partial_update(self, newer: Self) -> Self {
        Self {
            id: newer.id,
            name: newer.name,
            annotation: newer.annotation.or(self.annotation),
            tags: newer.tags.or(self.tags),
            aliases: newer.aliases.or(self.aliases),
            artist_type: newer.artist_type.or(self.artist_type),
            country: newer.country.or(self.country),
            gender: newer.gender.or(self.gender),
            genres: newer.genres.or(self.genres),
            life_span: newer.life_span.or(self.life_span),
            disambiguation: newer.disambiguation,
            recordings: newer.recordings.or(self.recordings),
            release_groups: newer.release_groups.or(self.release_groups),
            releases: newer.releases.or(self.releases),
            sort_name: newer.sort_name,
            works: newer.works.or(self.works),
            relations: newer.relations.or(self.relations),
        }
    }

    fn try_from_any(value: &AnyMusicBrainzEntity) -> Result<Arc<Self>, Error> {
        if let AnyMusicBrainzEntity::Artist(val) = value {
            return Ok(val.clone());
        }

        Err(Error::InvalidTypeConvertion(
            "MusicBrainzEntity".to_string(),
            "Artist".to_string(),
        ))
    }

    fn into_any(self: Arc<Self>) -> AnyMusicBrainzEntity {
        AnyMusicBrainzEntity::Artist(self)
    }

    fn into_arc_and_any(self) -> AnyMusicBrainzEntity {
        Arc::new(self).into_any()
    }
}
