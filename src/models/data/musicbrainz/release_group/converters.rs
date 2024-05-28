use itertools::Itertools;
use musicbrainz_rs::entity::release_group::ReleaseGroup as ReleaseGroupMS;

use crate::models::data::musicbrainz::release_group::ReleaseGroup;

impl From<ReleaseGroupMS> for ReleaseGroup {
    fn from(value: ReleaseGroupMS) -> Self {
        Self {
            id: value.id.into(),
            title: value.title,
            artist_credit: value
                .artist_credit
                .map(|artist_credits| artist_credits.into()),
            annotation: value.annotation,
            releases: value.releases.map(|releases| {
                releases
                    .into_iter()
                    .map(|release| release.id.into())
                    .collect_vec()
            }),
            genres: value.genres,
            aliases: value.aliases,
            tags: value.tags,
            disambiguation: value.disambiguation,
            primary_type: value.primary_type,
            first_release_date: value.first_release_date,
            secondary_type_ids: value.secondary_type_ids,
            secondary_types: value.secondary_types,
            primary_type_id: value.primary_type_id,
        }
    }
}
