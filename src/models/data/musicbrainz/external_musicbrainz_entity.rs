use extend::ext;
use musicbrainz_rs::entity::relations::RelationContent;

use crate::core::entity_traits::mbid::HasMBID;
use crate::models::data::musicbrainz::mbid::MBID;
use crate::models::data::musicbrainz::musicbrainz_entity::MusicBrainzEntity;
use crate::models::data::musicbrainz_database::MUSICBRAINZ_DATABASE;

pub type ExternalMusicBrainzEntity = RelationContent;
pub type FlattenedMBEntity = (MusicBrainzEntity, Vec<MusicBrainzEntity>);

#[ext]
pub impl ExternalMusicBrainzEntity {}

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
            .add_alias(&mbid.clone(), &self.0.get_mbid())
            .await?;

        for extra in self.1 {
            extra.save_to_cache().await?;
        }

        Ok(())
    }
}
