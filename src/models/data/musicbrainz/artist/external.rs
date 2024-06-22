use extend::ext;
use musicbrainz_rs::entity::artist::Artist;

use crate::core::entity_traits::has_id::HasID;
use crate::models::data::musicbrainz::external_musicbrainz_entity::ExternalMusicBrainzEntity;
use crate::models::data::musicbrainz::musicbrainz_entity::MusicBrainzEntity;
use crate::models::data::musicbrainz::recording::external::RecordingExt;
use crate::models::data::musicbrainz::relation::external::RelationContentExt;
use crate::models::data::musicbrainz::release::external::ReleaseExt;
use crate::models::data::musicbrainz::release_group::external::ReleaseGroupExt;
use crate::models::data::musicbrainz::work::external::WorkExt;

impl HasID for Artist {
    fn get_id(&self) -> String {
        self.id.to_string()
    }
}

#[ext]
pub impl Artist {
    fn flatten_main(&self) -> MusicBrainzEntity {
        super::Artist::from(self.clone()).into()
    }

    fn flatten_children(&self) -> Vec<MusicBrainzEntity> {
        let mut result: Vec<MusicBrainzEntity> = Vec::new();

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
        ExternalMusicBrainzEntity::Artist(Box::new(self))
    }
}
