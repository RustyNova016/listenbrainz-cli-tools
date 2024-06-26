use extend::ext;
use musicbrainz_rs::entity::relations::RelationContent;

use crate::core::entity_traits::mbid::{HasMBID, IsMbid};
use crate::models::data::musicbrainz::mbid::MBIDEnum;
use crate::models::data::musicbrainz::musicbrainz_entity::AnyMusicBrainzEntity;
use crate::models::data::musicbrainz_database::MUSICBRAINZ_DATABASE;
use crate::utils::println_cli;

pub type ExternalMusicBrainzEntity = RelationContent;
pub type FlattenedMBEntity = (AnyMusicBrainzEntity, Vec<AnyMusicBrainzEntity>);

#[ext]
pub impl ExternalMusicBrainzEntity {}

#[ext]
pub impl FlattenedMBEntity {
    async fn insert_into_cache_with_alias(self, mbid: &MBIDEnum) -> color_eyre::Result<()> {
        println_cli("add alias 1");
        MUSICBRAINZ_DATABASE
            .add_alias(mbid, &self.0.get_mbid())
            .await?;

        self.insert_into_cache().await
    }

    async fn insert_into_cache(self) -> color_eyre::Result<()> {
        //println_cli("Saving into cache");
        //Let's take care of the main data
        let mbid = self.0.get_mbid();

        println_cli("first cahce insert");
        self.0.update_cache().await?;

        MUSICBRAINZ_DATABASE
            .add_alias(&mbid.clone().into_mbid(), &self.0.get_mbid())
            .await?;

        println_cli("save others");

        for extra in self.1 {
            extra.update_cache().await?;
        }

        Ok(())
    }
}
