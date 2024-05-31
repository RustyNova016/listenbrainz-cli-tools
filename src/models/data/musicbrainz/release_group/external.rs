use extend::ext;
use musicbrainz_rs::entity::release_group::ReleaseGroup;

use crate::core::entity_traits::has_id::HasID;
use crate::models::data::musicbrainz::artist::external::ArtistExt;
use crate::models::data::musicbrainz::external_musicbrainz_entity::ExternalMusicBrainzEntity;
use crate::models::data::musicbrainz::musicbrainz_entity::MusicBrainzEntity;
use crate::models::data::musicbrainz::relation::external::RelationContentExt;
use crate::models::data::musicbrainz::release::external::ReleaseExt;

impl HasID for ReleaseGroup {
    fn get_id(&self) -> String {
        self.id.to_string()
    }
}

#[ext]
pub impl ReleaseGroup {
    fn flatten_main(&self) -> MusicBrainzEntity {
        MusicBrainzEntity::ReleaseGroup(super::ReleaseGroup::from(self.clone()))
    }

    fn flatten_children(&self) -> Vec<MusicBrainzEntity> {
        let mut result: Vec<MusicBrainzEntity> = Vec::new();

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
        ExternalMusicBrainzEntity::ReleaseGroup(Box::new(self))
    }
}
