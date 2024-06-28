use std::sync::Arc;

use crate::core::caching::musicbrainz_cache::MusicbrainzCacheLegacy;
use crate::core::entity_traits::has_id::HasID;
use crate::core::entity_traits::mb_cached::MBCached;
use crate::core::entity_traits::mbid::HasMBID;
use crate::core::entity_traits::update::Updatable;
use crate::models::data::musicbrainz::recording::mbid::RecordingMBID;
use crate::models::data::musicbrainz::recording::Recording;
use crate::models::data::musicbrainz_database_legacy::MUSICBRAINZ_DATABASE_LEGACY;

impl HasID for Recording {
    fn get_id(&self) -> String {
        self.id.to_string()
    }
}

impl HasMBID<RecordingMBID> for Recording {
    fn get_mbid(&self) -> RecordingMBID {
        self.id.clone()
    }
}

impl MBCached<RecordingMBID> for Recording {
    fn get_cache() -> Arc<MusicbrainzCacheLegacy<RecordingMBID, Self>> {
        MUSICBRAINZ_DATABASE_LEGACY.recordings().clone()
    }
}
impl Updatable for Recording {}