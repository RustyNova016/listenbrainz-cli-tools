use std::sync::Arc;

use crate::core::caching::musicbrainz_cache::MusicbrainzCacheLegacy;
use crate::core::entity_traits::has_id::HasID;
use crate::core::entity_traits::mb_cached::MBCached;
use crate::core::entity_traits::mbid::HasMBID;

use crate::core::entity_traits::update::Updatable;
use crate::models::data::musicbrainz::release::mbid::ReleaseMBID;
use crate::models::data::musicbrainz_database_legacy::MUSICBRAINZ_DATABASE_LEGACY;

use super::Release;

impl HasID for Release {
    fn get_id(&self) -> String {
        self.id.to_string()
    }
}

impl HasMBID<ReleaseMBID> for Release {
    fn get_mbid(&self) -> ReleaseMBID {
        self.id.clone()
    }
}

impl MBCached<ReleaseMBID> for Release {
    fn get_cache() -> Arc<MusicbrainzCacheLegacy<ReleaseMBID, Self>> {
        MUSICBRAINZ_DATABASE_LEGACY.releases().clone()
    }
}

impl Updatable for Release {}
