use std::sync::Arc;

use crate::core::caching::musicbrainz::musicbrainz_cache::MusicbrainzCache;
use crate::models::data::musicbrainz::entity::any::any_musicbrainz_entity::AnyMusicBrainzEntity;
use crate::models::data::musicbrainz::entity::entity_kind::MusicbrainzEntityKind;
use crate::models::data::musicbrainz::entity::traits::MusicBrainzEntity;
use crate::models::data::musicbrainz::external_musicbrainz_entity::ExternalMusicBrainzEntity;
use crate::models::data::musicbrainz::mbid::state_id::state::PrimaryMBID;
use crate::models::data::musicbrainz::mbid::state_id::MBIDState;
use crate::models::data::musicbrainz::mbid::state_id::MBIDWithState;
use crate::models::data::musicbrainz_database::MUSICBRAINZ_DATABASE;
use crate::models::error::Error;
use crate::utils::println_mus;
use color_eyre::eyre::Context;
use musicbrainz_rs::entity::recording::Recording as RecordingMS;
use musicbrainz_rs::Fetch;

use super::external::RecordingExt;
use super::Recording;

impl MusicBrainzEntity for Recording {
    async fn fetch<S: MBIDState>(
        id: &MBIDWithState<Self, S>,
    ) -> color_eyre::Result<ExternalMusicBrainzEntity> {
        println_mus(format!("Getting data for recording MBID: {}", &id));

        color_eyre::eyre::Ok(
            RecordingMS::fetch()
                .id(id)
                .with_artists()
                .with_releases()
                .with_work_relations()
                .with_aliases()
                .with_work_level_relations()
                .execute()
                .await
                .context("Failed to fetch recording from MusicBrainz")?
                .into_entity(),
        )
    }

    fn get_cache() -> Arc<MusicbrainzCache<Self>> {
        MUSICBRAINZ_DATABASE.recordings().clone()
    }

    fn get_mbid(&self) -> PrimaryMBID<Self> {
        PrimaryMBID::from(self.id.to_string().clone())
    }

    fn into_any(self: Arc<Self>) -> AnyMusicBrainzEntity {
        AnyMusicBrainzEntity::Recording(self)
    }

    fn try_from_any(value: &AnyMusicBrainzEntity) -> Result<Arc<Self>, Error> {
        match value {
            AnyMusicBrainzEntity::Recording(val) => Ok(val.clone()),
            _ => Err(Error::InvalidTypeConvertion(
                "AnyMusicBrainzEntity".to_string(),
                "Recording".to_string(),
            )),
        }
    }

    fn incremental_update(self, newer: Self) -> Self {
        Self {
            id: newer.id,
            title: newer.title,
            artist_credit: newer.artist_credit.or(self.artist_credit),
            releases: newer.releases.or(self.releases),
            isrcs: newer.isrcs.or(self.isrcs),
            disambiguation: newer.disambiguation.or(self.disambiguation),
            tags: newer.tags.or(self.tags),
            video: newer.video.or(self.video),
            length: newer.length.or(self.length),
            annotation: newer.annotation.or(self.annotation),
            genres: newer.genres.or(self.genres),
            aliases: self.aliases,
            relations: newer.relations.or(self.relations),
        }
    }

    fn get_kind() -> MusicbrainzEntityKind {
        MusicbrainzEntityKind::Recording
    }
}
