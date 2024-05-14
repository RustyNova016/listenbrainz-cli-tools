use musicbrainz_rs::entity::work::Work as WorkMS;

use super::Work;

impl From<WorkMS> for Work {
    fn from(value: WorkMS) -> Self {
        Self {
            id: value.id.into(),
            aliases: value.aliases,
            annotation: value.annotation,
            attributes: value.attributes,
            disambiguation: value.disambiguation,
            genres: value.genres,
            iswcs: value.iswcs,
            language: value.language,
            languages: value.languages,
            tags: value.tags,
            title: value.title,
            type_id: value.type_id,
            work_type: value.work_type,
        }
    }
}
