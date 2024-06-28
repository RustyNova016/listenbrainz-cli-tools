use std::sync::Arc;

use itertools::Itertools;

use crate::models::data::musicbrainz::entity::any_musicbrainz_entity::AnyMusicBrainzEntity;
use crate::models::data::musicbrainz::recording::Recording;
use crate::models::error::Error;

impl From<musicbrainz_rs::entity::recording::Recording> for Recording {
    fn from(recording: musicbrainz_rs::entity::recording::Recording) -> Self {
        Self {
            id: recording.id.into(),
            title: recording.title,
            artist_credit: recording.artist_credit.map(|coll| coll.into()),
            releases: recording.releases.map(|releases| {
                releases
                    .into_iter()
                    .map(|release| release.id.into())
                    .collect_vec()
            }),
            length: recording.length,
            video: recording.video,
            aliases: recording.aliases,
            genres: recording.genres,
            annotation: recording.annotation,
            tags: recording.tags,
            isrcs: recording.isrcs,
            disambiguation: recording.disambiguation,
            relations: recording
                .relations
                .map(|relations| relations.into_iter().map_into().collect_vec()),
        }
    }
}

impl From<AnyMusicBrainzEntity> for Result<Arc<Recording>, Error> {
    fn from(value: AnyMusicBrainzEntity) -> Self {
        if let AnyMusicBrainzEntity::Recording(val) = value {
            return Ok(val);
        }

        Err(Error::InvalidTypeConvertion(
            "MusicBrainzEntity".to_string(),
            "Recording".to_string(),
        ))
    }
}
