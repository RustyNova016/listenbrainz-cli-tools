use itertools::Itertools;

use super::Recording;

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
