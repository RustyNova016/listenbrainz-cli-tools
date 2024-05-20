use extend::ext;
use musicbrainz_rs::entity::relations::RelationContent;
use crate::models::data::musicbrainz::artist::external::ArtistExt;
use crate::models::data::musicbrainz::musicbrainz_entity::MusicBrainzEntity;
use crate::models::data::musicbrainz::recording::external::RecordingExt;
use crate::models::data::musicbrainz::release::external::ReleaseExt;
use crate::models::data::musicbrainz::release_group::external::ReleaseGroupExt;
use crate::models::data::musicbrainz::work::external::WorkExt;

pub type ExternalMusicBrainzEntity = RelationContent;

#[ext]
pub impl ExternalMusicBrainzEntity {
    fn flattened(&self) -> (MusicBrainzEntity, Vec<MusicBrainzEntity>) {
        (self.flatten_main(), self.flatten_children())
    }

    fn flatten_main(&self) -> MusicBrainzEntity {
        match self {
            Self::Artist(val) => val.flatten_main(),
            Self::Work(val) => val.flatten_main(),
            Self::ReleaseGroup(val) => val.flatten_main(),
            Self::Release(val) => val.flatten_main(),
            Self::Recording(val) => val.flatten_main(),
            _ => todo!("flatten_main of entity")
        }
    }

    fn flatten_children(&self) -> Vec<MusicBrainzEntity> {
        match self {
            Self::Artist(val) => val.flatten_children(),
            Self::Work(val) => val.flatten_children(),
            Self::ReleaseGroup(val) => val.flatten_children(),
            Self::Release(val) => val.flatten_children(),
            Self::Recording(val) => val.flatten_children(),
            _ => todo!("flatten_main of entity")
        }
    }
}

