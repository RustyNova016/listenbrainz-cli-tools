use extend::ext;
use musicbrainz_rs::entity::work::Work;

use crate::core::entity_traits::has_id::HasID;
use crate::models::data::musicbrainz::external_musicbrainz_entity::ExternalMusicBrainzEntity;
use crate::models::data::musicbrainz::musicbrainz_entity::AnyMusicBrainzEntity;
use crate::models::data::musicbrainz::relation::external::RelationContentExt;

impl HasID for Work {
    fn get_id(&self) -> String {
        self.id.to_string()
    }
}

#[ext]
pub impl Work {
    fn flatten_main(&self) -> AnyMusicBrainzEntity {
        super::Work::from(self.clone()).into()
    }

    fn flatten_children(&self) -> Vec<AnyMusicBrainzEntity> {
        let mut result: Vec<AnyMusicBrainzEntity> = Vec::new();

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

    fn flattened(&self) -> (AnyMusicBrainzEntity, Vec<AnyMusicBrainzEntity>) {
        (self.flatten_main(), self.flatten_children())
    }

    fn into_entity(self) -> ExternalMusicBrainzEntity {
        ExternalMusicBrainzEntity::Work(Box::new(self))
    }
}
