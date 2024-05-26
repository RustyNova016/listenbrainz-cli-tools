use extend::ext;
use musicbrainz_rs::entity::relations::RelationContent;

use crate::core::entity_traits::mbid::{HasMBID, IsMbid};
use crate::models::data::musicbrainz::artist::external::ArtistExt;
use crate::models::data::musicbrainz::mbid::MBID;
use crate::models::data::musicbrainz::musicbrainz_entity::MusicBrainzEntity;
use crate::models::data::musicbrainz::recording::external::RecordingExt;
use crate::models::data::musicbrainz::release::external::ReleaseExt;
use crate::models::data::musicbrainz::release_group::external::ReleaseGroupExt;
use crate::models::data::musicbrainz::work::external::WorkExt;
use crate::models::data::musicbrainz_database::MUSICBRAINZ_DATABASE;

pub type ExternalMusicBrainzEntity = RelationContent;
pub type FlattenedMBEntity = (MusicBrainzEntity, Vec<MusicBrainzEntity>);

#[ext]
pub impl ExternalMusicBrainzEntity {
    fn flattened(&self) -> FlattenedMBEntity {
        (self.flatten_main(), self.flatten_children())
    }

    fn flatten_main(&self) -> MusicBrainzEntity {
        match self {
            Self::Artist(val) => val.flatten_main(),
            Self::Work(val) => val.flatten_main(),
            Self::ReleaseGroup(val) => val.flatten_main(),
            Self::Release(val) => val.flatten_main(),
            Self::Recording(val) => val.flatten_main(),
            _ => todo!("flatten_main of entity"),
        }
    }

    fn flatten_children(&self) -> Vec<MusicBrainzEntity> {
        match self {
            Self::Artist(val) => val.flatten_children(),
            Self::Work(val) => val.flatten_children(),
            Self::ReleaseGroup(val) => val.flatten_children(),
            Self::Release(val) => val.flatten_children(),
            Self::Recording(val) => val.flatten_children(),
            _ => todo!("flatten_main of entity"),
        }
    }
}

#[ext]
pub impl FlattenedMBEntity {
    async fn insert_into_cache_with_alias(self, mbid: &MBID) -> color_eyre::Result<()> {
        MUSICBRAINZ_DATABASE
            .add_alias(mbid, &self.0.get_mbid())
            .await?;

        self.insert_into_cache().await
    }

    async fn insert_into_cache(self) -> color_eyre::Result<()> {
        //println_cli("Saving into cache");
        //Let's take care of the main data
        let mbid = self.0.get_mbid();

        self.0.save_to_cache().await?;

        MUSICBRAINZ_DATABASE
            .add_alias(&mbid.clone().into_mbid(), &self.0.get_mbid())
            .await?;

        for extra in self.1 {
            extra.save_to_cache().await?;
        }

        Ok(())
    }
}
