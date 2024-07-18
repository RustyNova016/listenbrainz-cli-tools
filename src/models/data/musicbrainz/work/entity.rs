use std::sync::Arc;

use crate::core::caching::musicbrainz::musicbrainz_cache::MusicbrainzCache;
use crate::models::data::musicbrainz::entity::any::any_musicbrainz_entity::AnyMusicBrainzEntity;
use crate::models::data::musicbrainz::entity::traits::MusicBrainzEntity;
use crate::models::data::musicbrainz::external_musicbrainz_entity::ExternalMusicBrainzEntity;
use crate::models::data::musicbrainz::mbid::state_id::state::PrimaryMBID;
use crate::models::data::musicbrainz::mbid::state_id::MBIDState;
use crate::models::data::musicbrainz::mbid::state_id::MBIDWithState;
use crate::models::data::musicbrainz_database::MUSICBRAINZ_DATABASE;
use crate::models::error::Error;
use crate::utils::println_mus;
use color_eyre::eyre::Context;
use musicbrainz_rs::entity::work::Work as WorkMS;
use musicbrainz_rs::Fetch;

use super::external::WorkExt;
use super::Work;

impl MusicBrainzEntity for Work {
    async fn fetch<S: MBIDState>(
        id: &MBIDWithState<Self, S>,
    ) -> color_eyre::Result<ExternalMusicBrainzEntity> {
        println_mus(format!("Getting data for work MBID: {}", &id));

        Ok(WorkMS::fetch()
            .id(id)
            .with_aliases()
            .with_annotations()
            .with_artist_relations()
            .with_genres()
            .with_label_relations()
            .with_ratings()
            .with_recording_relations()
            .with_tags()
            .with_url_relations()
            .with_work_relations()
            .execute()
            .await
            .context("Failed to fetch work from MusicBrainz")?
            .into_entity())
    }

    fn get_cache() -> Arc<MusicbrainzCache<Self>> {
        MUSICBRAINZ_DATABASE.works().clone()
    }

    fn get_mbid(&self) -> PrimaryMBID<Self> {
        PrimaryMBID::from(self.id.to_string().clone())
    }

    fn into_any(self: Arc<Self>) -> AnyMusicBrainzEntity {
        AnyMusicBrainzEntity::Work(self)
    }

    fn try_from_any(value: &AnyMusicBrainzEntity) -> Result<Arc<Self>, Error> {
        match value {
            AnyMusicBrainzEntity::Work(val) => Ok(val.clone()),
            _ => Err(Error::InvalidTypeConvertion(
                "AnyMusicBrainzEntity".to_string(),
                "Work".to_string(),
            )),
        }
    }

    fn incremental_update(self, newer: Self) -> Self {
        Self {
            id: newer.id,
            disambiguation: newer.disambiguation,
            title: newer.title,
            tags: newer.tags.or(self.tags),
            aliases: newer.aliases.or(self.aliases),
            genres: newer.genres.or(self.genres),
            annotation: newer.annotation.or(self.annotation),
            attributes: newer.attributes.or(self.attributes),
            iswcs: newer.iswcs.or(self.iswcs),
            language: newer.language.or(self.language),
            languages: newer.languages.or(self.languages),
            type_id: newer.type_id.or(self.type_id),
            //work_type: newer.work_type.or(self.work_type),
            relations: newer.relations.or(self.relations),
        }
    }

    fn get_kind() -> crate::models::data::musicbrainz::entity::entity_kind::MusicbrainzEntityKind {
        crate::models::data::musicbrainz::entity::entity_kind::MusicbrainzEntityKind::Work
    }
}
