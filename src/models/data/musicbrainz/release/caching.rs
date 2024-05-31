use std::sync::Arc;

use crate::core::caching::musicbrainz_cache::MusicbrainzCache;
use crate::core::entity_traits::has_id::HasID;
use crate::core::entity_traits::mb_cached::MBCached;
use crate::core::entity_traits::mbid::HasMBID;
use crate::core::entity_traits::updatable::Updatable;
use crate::models::data::musicbrainz::release::mbid::ReleaseMBID;
use crate::models::data::musicbrainz_database::MUSICBRAINZ_DATABASE;

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
    fn get_cache() -> Arc<MusicbrainzCache<ReleaseMBID, Self>> {
        MUSICBRAINZ_DATABASE.releases().clone()
    }
}

impl Updatable for Release {
    fn update(self, newer: Self) -> Self {
        Self {
            annotation: newer.annotation.or(self.annotation),
            barcode: newer.barcode.or(self.barcode),
            country: newer.country.or(self.country),
            disambiguation: newer.disambiguation.or(self.disambiguation),
            media: newer.media.or(self.media),
            packaging_id: newer.packaging_id.or(self.packaging_id),
            status_id: newer.status_id.or(self.status_id),
            title: newer.title,
            id: newer.id,
            artist_credit: newer.artist_credit.or(self.artist_credit),
            release_group: newer.release_group.or(self.release_group),
            relations: newer.relations.or(self.relations),
            aliases: newer.aliases.or(self.aliases),
            date: newer.date.or(self.date),
            genres: newer.genres.or(self.genres),
            packaging: newer.packaging.or(self.packaging),
            //quality: newer.quality.or(self.quality),
            status: newer.status.or(self.status),
            tags: newer.tags.or(self.tags),
        }
    }
}
