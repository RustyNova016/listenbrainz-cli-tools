use extend::ext;
use musicbrainz_rs::entity::work::Work;

use crate::core::entity_traits::cached::Cached;
use crate::core::entity_traits::has_id::HasID;
use crate::core::entity_traits::insertable::Insertable;
use crate::core::entity_traits::insertable_children::InsertableWithChildren;
use crate::models::data::musicbrainz::external_musicbrainz_entity::{
    ExternalMusicBrainzEntity, ExternalMusicBrainzEntityExt,
};
use crate::models::data::musicbrainz::musicbrainz_entity::MusicBrainzEntity;

impl HasID for Work {
    fn get_id(&self) -> String {
        self.id.to_string()
    }
}

impl Insertable for Work {
    async fn insert_into_cache_as(&self, key: String) -> color_eyre::Result<()> {
        crate::models::data::musicbrainz::work::Work::get_cache()
            .update(&key, self.clone().into())
            .await
    }
}

impl InsertableWithChildren for Work {
    async fn insert_with_children(&self, key: String) -> color_eyre::Result<()> {
        self.insert_into_cache_as(key).await?;

        Ok(())
    }
}

#[ext]
pub impl Work {
    fn flatten_main(&self) -> MusicBrainzEntity {
        MusicBrainzEntity::Work(super::Work::from(self.clone()))
    }

    fn flatten_children(&self) -> Vec<MusicBrainzEntity> {
        let mut result: Vec<MusicBrainzEntity> = Vec::new();

        if let Some(relations) = self.relations.clone() {
            for relation in relations {
                result.push(relation.content.flatten_main());
                result.extend(relation.content.flatten_children());
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
