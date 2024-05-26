use std::sync::Arc;

use crate::core::caching::musicbrainz_cache::MusicbrainzCache;
use crate::core::entity_traits::has_id::HasID;
use crate::core::entity_traits::mb_cached::MBCached;
use crate::core::entity_traits::mbid::HasMBID;
use crate::core::entity_traits::updatable::Updatable;
use crate::models::data::musicbrainz::work::mbid::WorkMBID;
use crate::models::data::musicbrainz_database::MUSICBRAINZ_DATABASE;

use super::Work;

impl HasID for Work {
    fn get_id(&self) -> String {
        self.id.to_string()
    }
}

impl HasMBID<WorkMBID> for Work {
    fn get_mbid(&self) -> WorkMBID {
        self.id.clone()
    }
}

impl MBCached<WorkMBID> for Work {
    fn get_cache() -> Arc<MusicbrainzCache<WorkMBID, Self>> {
        MUSICBRAINZ_DATABASE.works().clone()
    }
}

impl Updatable for Work {
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
}
