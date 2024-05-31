use std::sync::Arc;

use crate::core::caching::musicbrainz_cache::MusicbrainzCache;
use crate::core::entity_traits::has_id::HasID;
use crate::core::entity_traits::mb_cached::MBCached;
use crate::core::entity_traits::mbid::HasMBID;
use crate::core::entity_traits::updatable::Updatable;
use crate::models::data::musicbrainz::release_group::mbid::ReleaseGroupMBID;
use crate::models::data::musicbrainz::release_group::ReleaseGroup;
use crate::models::data::musicbrainz_database::MUSICBRAINZ_DATABASE;

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
    fn get_cache() -> Arc<MusicbrainzCache<ReleaseGroupMBID, Self>> {
        MUSICBRAINZ_DATABASE.release_groups().clone()
    }
}

impl Updatable for ReleaseGroup {
    fn update(self, newer: Self) -> Self {
        Self {
            id: newer.id,
            secondary_types: newer.secondary_types,
            secondary_type_ids: newer.secondary_type_ids,
            disambiguation: newer.disambiguation,
            title: newer.title,
            primary_type_id: newer.primary_type_id.or(self.primary_type_id),
            first_release_date: newer.first_release_date.or(self.first_release_date),
            primary_type: newer.primary_type.or(self.primary_type),
            tags: newer.tags.or(self.tags),
            aliases: newer.aliases.or(self.aliases),
            genres: newer.genres.or(self.genres),
            releases: newer.releases.or(self.releases),
            annotation: newer.annotation.or(self.annotation),
            artist_credit: newer.artist_credit.or(self.artist_credit),
            relations: newer.relations.or(self.relations),
        }
    }
}
