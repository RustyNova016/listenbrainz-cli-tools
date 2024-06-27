use itertools::Itertools;
use musicbrainz_rs::entity::release::Release as ReleaseMS;

use crate::models::data::musicbrainz::musicbrainz_entity::MusicBrainzEntity;
use crate::models::error::Error;

use super::Release;

impl From<ReleaseMS> for Release {
    fn from(value: ReleaseMS) -> Self {
        Self {
            annotation: value.annotation,
            barcode: value.barcode,
            country: value.country,
            disambiguation: value.disambiguation,
            id: value.id.into(),
            media: value
                .media
                .map(|medias| medias.into_iter().map_into().collect_vec()),
            packaging_id: value.packaging_id,
            status_id: value.status_id,
            title: value.title,
            artist_credit: value
                .artist_credit
                .map(|artist_credits| artist_credits.into()),
            release_group: value
                .release_group
                .map(|release_group| release_group.id.into()),
            status: value.status,
            //quality: value.quality,
            packaging: value.packaging,
            genres: value.genres,
            date: value.date,
            aliases: value.aliases,
            relations: value
                .relations
                .map(|relations| relations.into_iter().map_into().collect_vec()),
            tags: value.tags,
        }
    }
}

impl From<MusicBrainzEntity> for Result<Release, Error> {
    fn from(value: MusicBrainzEntity) -> Self {
        if let MusicBrainzEntity::Release(val) = value {
            return Ok(val);
        }

        Err(Error::InvalidTypeConvertion(
            "MusicBrainzEntity".to_string(),
            "Release".to_string(),
        ))
    }
}
