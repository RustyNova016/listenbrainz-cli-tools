use derive_getters::Getters;
use musicbrainz_rs::entity::alias::Alias;
use musicbrainz_rs::entity::genre::Genre;
use musicbrainz_rs::entity::tag::Tag;
use musicbrainz_rs::entity::work::WorkAttribute;
use serde::Deserialize;
use serde::Serialize;

use crate::models::data::musicbrainz::entity::entity_kind::MusicbrainzEntityKind;
use crate::models::data::musicbrainz::entity::is_musicbrainz_entity::IsMusicbrainzEntity;
use crate::models::data::musicbrainz::mbid::generic_mbid::{MBIDSpe, PrimaryID};

use super::relation::Relation;

use self::mbid::WorkMBID;

pub mod caching;
pub mod converter;
pub mod external;
pub mod getters;
pub mod mbid;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Getters)]
pub struct Work {
    id: WorkMBID,
    title: String,
    type_id: Option<String>,
    //work_type: Option<WorkType>, //TODO: Wait for https://github.com/oknozor/musicbrainz_rs/pull/87
    language: Option<String>,
    languages: Option<Vec<String>>,
    iswcs: Option<Vec<String>>,
    attributes: Option<Vec<WorkAttribute>>,
    disambiguation: Option<String>,
    relations: Option<Vec<Relation>>,
    tags: Option<Vec<Tag>>,
    //rating: Option<Rating>, TODO: Create own struct that have Eq
    aliases: Option<Vec<Alias>>,
    genres: Option<Vec<Genre>>,
    annotation: Option<String>,
}

impl IsMusicbrainzEntity for Work {
    fn as_kind(&self) -> MusicbrainzEntityKind {
        MusicbrainzEntityKind::Work
    }

    fn get_mbidspe(&self) -> MBIDSpe<Self, PrimaryID> {
        MBIDSpe::from(self.id.to_string())
    }

    fn update(self, newer: Self) -> Self {
        Self {
            id: newer.id,
            disambiguation: newer.disambiguation,
            title: newer.title,
            tags: newer.tags.or(self.tags),
            aliases: newer.aliases.or(self.aliases),
            genres: newer.genres.or(self.genres),
            annotation: newer.annotation.or(self.annotation),
            attributes: newer.attributes.or(self.attributes),
            iswcs: newer.iswcs.or(self.iswcs),
            language: newer.language.or(self.language),
            languages: newer.languages.or(self.languages),
            type_id: newer.type_id.or(self.type_id),
            //work_type: newer.work_type.or(self.work_type),
            relations: newer.relations.or(self.relations),
        }
    }

    fn into_any(self) -> super::entity::any_musicbrainz_entity::AnyMusicBrainzEntity {
        self.into()
    }
}

impl Work {
    pub fn create_fake_work(id: String, title: String) -> Self {
        Self {
            id: id.into(),
            title,
            languages: Some(Vec::new()),
            iswcs: Some(Vec::new()),
            attributes: Some(Vec::new()),
            relations: Some(Vec::new()),
            tags: Some(Vec::new()),
            aliases: Some(Vec::new()),
            genres: Some(Vec::new()),
            annotation: Some(String::new()),
            disambiguation: Some(String::new()),
            language: Some(String::new()),
            type_id: Some(String::new()),
            //work_type: None,
        }
    }
}
