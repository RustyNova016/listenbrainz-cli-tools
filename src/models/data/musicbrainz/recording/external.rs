use extend::ext;
use musicbrainz_rs::entity::recording::Recording;

use crate::core::entity_traits::cached::Cached;
use crate::core::entity_traits::has_id::HasID;
use crate::core::entity_traits::insertable::{Insertable, IsAutoInsertable};
use crate::core::entity_traits::insertable_children::InsertableWithChildren;
use crate::models::data::musicbrainz::artist::external::ArtistExt;
use crate::models::data::musicbrainz::external_musicbrainz_entity::{ExternalMusicBrainzEntity, ExternalMusicBrainzEntityExt};
use crate::models::data::musicbrainz::musicbrainz_entity::MusicBrainzEntity;
use crate::models::data::musicbrainz::release::external::ReleaseExt;

impl HasID for Recording {
    fn get_id(&self) -> String {
        self.id.to_string()
    }
}

impl Insertable for Recording {
    async fn insert_into_cache_as(&self, key: String) -> color_eyre::Result<()> {
        crate::models::data::musicbrainz::recording::Recording::get_cache()
            .update(&key, self.clone().into())
            .await?;

        Ok(())
    }
}

impl InsertableWithChildren for Recording {
    async fn insert_with_children(&self, key: String) -> color_eyre::Result<()> {
        self.insert_into_cache_as(key).await?;

        if let Some(data) = self.artist_credit.clone() {
            for item in &data {
                item.insert_into_cache().await?;
            }
        }

        if let Some(data) = self.releases.clone() {
            for item in &data {
                item.insert_into_cache().await?;
            }
        }

        Ok(())
    }
}

#[ext]
pub impl Recording {
    fn flatten_main(&self) -> MusicBrainzEntity {
        MusicBrainzEntity::Recording(super::Recording::from(self.clone()))
    }

    fn flatten_children(&self) -> Vec<MusicBrainzEntity> {
        let mut result: Vec<MusicBrainzEntity>  = Vec::new();

        if let Some(artist_credits) = self.artist_credit.clone() {
            for artist_credit in artist_credits {
                result.push(artist_credit.artist.flatten_main());
                result.extend(artist_credit.artist.flatten_children());
            }
        }

        if let Some(releases) = self.releases.clone() {
            for release in releases {
                result.push(release.flatten_main());
                result.extend(release.flatten_children());
            }
        }

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
        ExternalMusicBrainzEntity::Recording(Box::new(self))
    }
}