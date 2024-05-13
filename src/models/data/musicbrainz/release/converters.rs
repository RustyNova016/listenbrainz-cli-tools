use itertools::Itertools;
use musicbrainz_rs::entity::release::Release as ReleaseMS;

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
        }
    }
}
