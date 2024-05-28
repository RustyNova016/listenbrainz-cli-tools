use extend::ext;
use musicbrainz_rs::entity::release::Release;

use crate::core::entity_traits::has_id::HasID;
use crate::models::data::musicbrainz::artist::external::ArtistExt;
use crate::models::data::musicbrainz::external_musicbrainz_entity::ExternalMusicBrainzEntity;
use crate::models::data::musicbrainz::musicbrainz_entity::MusicBrainzEntity;
use crate::models::data::musicbrainz::relation::external::RelationContentExt;
use crate::models::data::musicbrainz::release_group::external::ReleaseGroupExt;

impl HasID for Release {
    fn get_id(&self) -> String {
        self.id.to_string()
    }
}

#[ext]
pub impl Release {
    fn flatten_main(&self) -> MusicBrainzEntity {
        MusicBrainzEntity::Release(super::Release::from(self.clone()))
    }

    fn flatten_children(&self) -> Vec<MusicBrainzEntity> {
        let mut result: Vec<MusicBrainzEntity> = Vec::new();

        if let Some(artist_credits) = self.artist_credit.clone() {
            for artist_credit in artist_credits {
                result.push(artist_credit.artist.flatten_main());
                result.extend(artist_credit.artist.flatten_children());
            }
        }

        if let Some(release_group) = self.release_group.clone() {
            result.push(release_group.flatten_main());
            result.extend(release_group.flatten_children());
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
        ExternalMusicBrainzEntity::Release(Box::new(self))
    }
}
