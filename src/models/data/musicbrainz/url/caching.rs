use std::sync::Arc;
use crate::core::caching::musicbrainz_cache::MusicbrainzCache;
use crate::core::entity_traits::has_id::HasID;
use crate::core::entity_traits::mb_cached::MBCached;
use crate::core::entity_traits::mbid::HasMBID;
use crate::core::entity_traits::updatable::Updatable;
use crate::models::data::musicbrainz::url::mbid::URLMBID;
use crate::models::data::musicbrainz::url::URL;
use crate::models::data::musicbrainz::work::mbid::WorkMBID;
use crate::models::data::musicbrainz::work::Work;
use crate::models::data::musicbrainz_database::MUSICBRAINZ_DATABASE;

impl HasID for URL {
    fn get_id(&self) -> String {
        self.id.to_string()
    }
}

impl HasMBID<URLMBID> for URL {
    fn get_mbid(&self) -> URLMBID {
        self.id.clone()
    }
}

impl MBCached<URLMBID> for URL {
    fn get_cache() -> Arc<MusicbrainzCache<URLMBID, Self>> {
        MUSICBRAINZ_DATABASE.url().clone()
    }
}

impl Updatable for URL {
    fn update(self, newer: Self) -> Self {
        Self {
            id: newer.id,
            resource: newer.resource,
            tags: newer.tags.or(self.tags)
        }
    }
}
