use extend::ext;
use musicbrainz_rs::entity::relations::RelationContent;

use crate::core::entity_traits::mbid::{HasMBID, IsMbid};
use crate::models::data::musicbrainz::mbid::MBID;
use crate::models::data::musicbrainz::musicbrainz_entity::MusicBrainzEntity;
use crate::models::data::musicbrainz_database::MUSICBRAINZ_DATABASE;
use crate::ErrorKind;

pub type ExternalMusicBrainzEntity = RelationContent;
pub type FlattenedMBEntity = (MusicBrainzEntity, Vec<MusicBrainzEntity>);

#[ext]
pub impl ExternalMusicBrainzEntity {}

#[ext]
pub impl FlattenedMBEntity {
    async fn insert_into_cache_with_alias(self, mbid: &MBID) -> Result<(), ErrorKind> {
        MUSICBRAINZ_DATABASE
            .add_alias(mbid, &self.0.get_mbid())
            .await?;

        self.insert_into_cache().await
    }

    async fn insert_into_cache(self) -> Result<(), ErrorKind> {
        //println_cli("Saving into cache");
        //Let's take care of the main data
        let mbid = self.0.get_mbid();

        self.0.save_to_cache().await.map_err(ErrorKind::CacheError)?;

        MUSICBRAINZ_DATABASE
            .add_alias(&mbid.clone().into_mbid(), &self.0.get_mbid())
            .await?;

        for extra in self.1 {
            extra.save_to_cache().await.map_err(ErrorKind::CacheError)?;
        }

        Ok(())
    }
}
