use std::sync::Arc;

use crate::core::caching::musicbrainz_cache::MusicbrainzCacheLegacy;
use crate::core::entity_traits::has_id::HasID;
use crate::core::entity_traits::mb_cached::MBCached;
use crate::core::entity_traits::mbid::HasMBID;
use crate::models::data::musicbrainz::artist::mbid::ArtistMBID;
use crate::models::data::musicbrainz::artist::Artist;
use crate::models::data::musicbrainz_database_legacy::MUSICBRAINZ_DATABASE_LEGACY;

impl HasID for Artist {
    fn get_id(&self) -> String {
        self.id.to_string()
    }
}

impl HasMBID<ArtistMBID> for Artist {
    fn get_mbid(&self) -> ArtistMBID {
        self.id.clone()
    }
}

impl MBCached<ArtistMBID> for Artist {
    fn get_cache() -> Arc<MusicbrainzCacheLegacy<ArtistMBID, Self>> {
        MUSICBRAINZ_DATABASE_LEGACY.artists().clone()
    }
}
