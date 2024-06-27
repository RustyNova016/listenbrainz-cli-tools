use itertools::Itertools;
use musicbrainz_rs::entity::release_group::ReleaseGroup as ReleaseGroupMS;

use crate::models::data::musicbrainz::musicbrainz_entity::MusicBrainzEntity;
use crate::models::data::musicbrainz::release_group::ReleaseGroup;
use crate::models::error::Error;

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
            relations: value
                .relations
                .map(|relations| relations.into_iter().map_into().collect_vec()),
        }
    }
}

impl ReleaseGroup {
    pub fn try_from_entity(value: MusicBrainzEntity) -> Result<Self, Error> {
        Self::try_from(value)
    }
}

impl TryFrom<MusicBrainzEntity> for ReleaseGroup {
    type Error = Error;

    fn try_from(value: MusicBrainzEntity) -> Result<Self, Self::Error> {
        if let MusicBrainzEntity::ReleaseGroup(val) = value {
            return Ok(val);
        }

        Err(Error::InvalidTypeConvertion(
            "MusicBrainzEntity".to_string(),
            "ReleaseGroup".to_string(),
        ))
    }
}
