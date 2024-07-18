use extend::ext;
use musicbrainz_rs::entity::work::Work;

use crate::models::data::musicbrainz::external_musicbrainz_entity::ExternalMusicBrainzEntity;
use crate::models::data::musicbrainz::musicbrainz_entity::MusicBrainzEntity;
use crate::models::data::musicbrainz::relation::external::RelationContentExt;

#[ext]
pub impl Work {
    fn flatten_main(&self) -> MusicBrainzEntity {
        MusicBrainzEntity::Work(super::Work::from(self.clone()))
    }

    fn flatten_children(&self) -> Vec<MusicBrainzEntity> {
        let mut result: Vec<MusicBrainzEntity> = Vec::new();

        if let Some(relations) = self.relations.clone() {
            for relation in relations {
                if let Ok(res) = relation.content.flatten_main() {
                    result.push(res);
                }
                if let Ok(res) = relation.content.flatten_children() {
                    result.extend(res);
                }
            }
        }

        result
    }

    fn flattened(&self) -> (MusicBrainzEntity, Vec<MusicBrainzEntity>) {
        (self.flatten_main(), self.flatten_children())
    }

    fn into_entity(self) -> ExternalMusicBrainzEntity {
        ExternalMusicBrainzEntity::Work(Box::new(self))
    }
}
