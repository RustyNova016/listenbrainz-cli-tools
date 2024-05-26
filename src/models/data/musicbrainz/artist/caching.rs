use std::sync::Arc;

use crate::core::caching::musicbrainz_cache::MusicbrainzCache;
use crate::core::entity_traits::has_id::HasID;
use crate::core::entity_traits::mb_cached::MBCached;
use crate::core::entity_traits::mbid::HasMBID;
use crate::core::entity_traits::updatable::Updatable;
use crate::models::data::musicbrainz::artist::mbid::ArtistMBID;
use crate::models::data::musicbrainz::artist::Artist;
use crate::models::data::musicbrainz_database::MUSICBRAINZ_DATABASE;

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
    fn get_cache() -> Arc<MusicbrainzCache<ArtistMBID, Self>> {
        MUSICBRAINZ_DATABASE.artists().clone()
    }
}

impl Updatable for Artist {
    fn update(self, newer: Self) -> Self {
        Self {
            id: newer.id,
            name: newer.name,
            annotation: newer.annotation.or(self.annotation),
            tags: newer.tags.or(self.tags),
            aliases: newer.aliases.or(self.aliases),
            artist_type: newer.artist_type.or(self.artist_type),
            country: newer.country.or(self.country),
            gender: newer.gender.or(self.gender),
            genres: newer.genres.or(self.genres),
            life_span: newer.life_span.or(self.life_span),
            disambiguation: newer.disambiguation,
            recordings: newer.recordings.or(self.recordings),
            release_groups: newer.release_groups.or(self.release_groups),
            releases: newer.releases.or(self.releases),
            sort_name: newer.sort_name,
            works: newer.works.or(self.works),
            relations: newer.relations.or(self.relations),
        }
    }
}
