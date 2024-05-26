use color_eyre::eyre::eyre;
use extend::ext;
use musicbrainz_rs::entity::relations::RelationContent;

use crate::models::data::musicbrainz::artist::external::ArtistExt;
use crate::models::data::musicbrainz::external_musicbrainz_entity::FlattenedMBEntity;
use crate::models::data::musicbrainz::musicbrainz_entity::MusicBrainzEntity;
use crate::models::data::musicbrainz::recording::external::RecordingExt;
use crate::models::data::musicbrainz::release::external::ReleaseExt;
use crate::models::data::musicbrainz::release_group::external::ReleaseGroupExt;
use crate::models::data::musicbrainz::work::external::WorkExt;

#[ext]
pub impl RelationContent {
    fn flattened(&self) -> FlattenedMBEntity {
        (
            self.flatten_main().expect("Missing implementation"),
            self.flatten_children().expect("Missing implementation"),
        )
    }

    fn flattened_result(&self) -> color_eyre::Result<FlattenedMBEntity> {
        Ok((self.flatten_main()?, self.flatten_children()?))
    }

    fn flatten_main(&self) -> color_eyre::Result<MusicBrainzEntity> {
        //TODO: Remove when implemented
        match self {
            RelationContent::Artist(val) => Ok(val.flatten_main()),
            RelationContent::Area(_)
            | RelationContent::Place(_)
            | RelationContent::Event(_)
            | RelationContent::Label(_)
            | RelationContent::Series(_)
            | RelationContent::Url(_) => Err(eyre!("Not yet implemented")),
            RelationContent::Recording(val) => Ok(val.flatten_main()),
            RelationContent::Release(val) => Ok(val.flatten_main()),
            RelationContent::ReleaseGroup(val) => Ok(val.flatten_main()),
            RelationContent::Work(val) => Ok(val.flatten_main()),
        }
    }

    fn flatten_children(&self) -> color_eyre::Result<Vec<MusicBrainzEntity>> {
        //TODO: Remove when implemented
        match self {
            RelationContent::Artist(val) => Ok(val.flatten_children()),
            RelationContent::Area(_)
            | RelationContent::Label(_)
            | RelationContent::Place(_)
            | RelationContent::Series(_)
            | RelationContent::Event(_)
            | RelationContent::Url(_) => Err(eyre!("Not yet implemented")),
            RelationContent::Recording(val) => Ok(val.flatten_children()),
            RelationContent::Release(val) => Ok(val.flatten_children()),
            RelationContent::ReleaseGroup(val) => Ok(val.flatten_children()),
            RelationContent::Work(val) => Ok(val.flatten_children()),
        }
    }
}
