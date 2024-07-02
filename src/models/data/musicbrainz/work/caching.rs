use std::sync::Arc;

use crate::core::caching::musicbrainz_cache::MusicbrainzCacheLegacy;
use crate::core::entity_traits::has_id::HasID;
use crate::core::entity_traits::mb_cached::MBCached;
use crate::core::entity_traits::mbid::HasMBID;

use crate::core::entity_traits::update::Updatable;
use crate::models::data::musicbrainz::work::mbid::WorkMBID;
use crate::models::data::musicbrainz_database_legacy::MUSICBRAINZ_DATABASE_LEGACY;

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
    fn get_cache() -> Arc<MusicbrainzCacheLegacy<WorkMBID, Self>> {
        MUSICBRAINZ_DATABASE_LEGACY.works().clone()
    }
}

impl Updatable for Work {}
