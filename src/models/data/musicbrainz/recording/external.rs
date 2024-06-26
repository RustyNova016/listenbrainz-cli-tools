use extend::ext;
use musicbrainz_rs::entity::recording::Recording;

use crate::core::entity_traits::has_id::HasID;
use crate::models::data::musicbrainz::artist::external::ArtistExt;
use crate::models::data::musicbrainz::external_musicbrainz_entity::ExternalMusicBrainzEntity;
use crate::models::data::musicbrainz::musicbrainz_entity::AnyMusicBrainzEntity;
use crate::models::data::musicbrainz::relation::external::RelationContentExt;
use crate::models::data::musicbrainz::release::external::ReleaseExt;

impl HasID for Recording {
    fn get_id(&self) -> String {
        self.id.to_string()
    }
}

#[ext]
pub impl Recording {
    fn flatten_main(&self) -> AnyMusicBrainzEntity {
        super::Recording::from(self.clone()).into()
    }

    fn flatten_children(&self) -> Vec<AnyMusicBrainzEntity> {
        let mut result: Vec<AnyMusicBrainzEntity> = Vec::new();

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

    fn flattened(&self) -> (AnyMusicBrainzEntity, Vec<AnyMusicBrainzEntity>) {
        (self.flatten_main(), self.flatten_children())
    }

    fn into_entity(self) -> ExternalMusicBrainzEntity {
        ExternalMusicBrainzEntity::Recording(Box::new(self))
    }
}
