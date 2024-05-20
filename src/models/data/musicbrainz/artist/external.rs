use extend::ext;
use crate::core::entity_traits::cached::Cached;
use crate::core::entity_traits::has_id::HasID;
use crate::core::entity_traits::insertable::{Insertable, IsAutoInsertable};
use crate::core::entity_traits::insertable_children::InsertableWithChildren;
use musicbrainz_rs::entity::artist::Artist;
use crate::models::data::musicbrainz::external_musicbrainz_entity::{ExternalMusicBrainzEntity, ExternalMusicBrainzEntityExt};
use crate::models::data::musicbrainz::musicbrainz_entity::MusicBrainzEntity;
use crate::models::data::musicbrainz::recording::external::RecordingExt;
use crate::models::data::musicbrainz::release::external::ReleaseExt;
use crate::models::data::musicbrainz::work::external::WorkExt;
use crate::models::data::musicbrainz::release_group::external::ReleaseGroupExt;


impl HasID for Artist {
    fn get_id(&self) -> String {
        self.id.to_string()
    }
}

impl Insertable for Artist {
    async fn insert_into_cache_as(&self, key: String) -> color_eyre::Result<()> {
        crate::models::data::musicbrainz::artist::Artist::get_cache()
            .update(&key, self.clone().into())
            .await
    }
}

impl InsertableWithChildren for Artist {
    async fn insert_with_children(&self, key: String) -> color_eyre::Result<()> {
        self.insert_into_cache_as(key).await?;

        if let Some(recordings) = self.recordings.clone() {
            for recording in recordings {
                recording.insert_into_cache().await?;
            }
        }

        if let Some(releases) = self.releases.clone() {
            for release in releases {
                release.insert_into_cache().await?;
            }
        }

        Ok(())
    }
}

#[ext]
pub impl Artist {
    fn flatten_main(&self) -> MusicBrainzEntity {
        MusicBrainzEntity::Artist(super::Artist::from(self.clone()))
    }

    fn flatten_children(&self) -> Vec<MusicBrainzEntity> {
        let mut result: Vec<MusicBrainzEntity>  = Vec::new();

        if let Some(recordings) = self.recordings.clone() {
            for recording in recordings {
                result.push(recording.flatten_main());
                result.extend(recording.flatten_children());
            }
        }

        if let Some(releases) = self.releases.clone() {
            for release in releases {
                result.push(release.flatten_main());
                result.extend(release.flatten_children());
            }
        }

        if let Some(release_groups) = self.release_groups.clone() {
            for release_group in release_groups {
                result.push(release_group.flatten_main());
                result.extend(release_group.flatten_children());
            }
        }

        if let Some(works) = self.works.clone() {
            for work in works {
                result.push(work.flatten_main());
                result.extend(work.flatten_children());
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
        ExternalMusicBrainzEntity::Artist(Box::new(self))
    }
}