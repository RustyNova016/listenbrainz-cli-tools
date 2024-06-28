use std::sync::Arc;

use crate::core::caching::musicbrainz_cache::MusicbrainzCacheLegacy;
use crate::core::entity_traits::has_id::HasID;
use crate::core::entity_traits::mb_cached::MBCached;
use crate::core::entity_traits::mbid::HasMBID;

use crate::core::entity_traits::update::Updatable;
use crate::models::data::musicbrainz::release_group::mbid::ReleaseGroupMBID;
use crate::models::data::musicbrainz::release_group::ReleaseGroup;
use crate::models::data::musicbrainz_database_legacy::MUSICBRAINZ_DATABASE_LEGACY;

impl HasID for ReleaseGroup {
    fn get_id(&self) -> String {
        self.id.to_string()
    }
}

impl HasMBID<ReleaseGroupMBID> for ReleaseGroup {
    fn get_mbid(&self) -> ReleaseGroupMBID {
        self.id.clone()
    }
}

impl MBCached<ReleaseGroupMBID> for ReleaseGroup {
    fn get_cache() -> Arc<MusicbrainzCacheLegacy<ReleaseGroupMBID, Self>> {
        MUSICBRAINZ_DATABASE_LEGACY.release_groups().clone()
    }
}

impl Updatable for ReleaseGroup {}
