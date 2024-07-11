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
use musicbrainz_rs::entity::release::Release as ReleaseMS;
use musicbrainz_rs::Fetch;

use super::external::ReleaseExt;
use super::Release;

impl MusicBrainzEntity for Release {
    async fn fetch<S: MBIDState>(
        id: &MBIDWithState<Self, S>,
    ) -> color_eyre::Result<ExternalMusicBrainzEntity> {
        println_mus(format!("Getting data for release MBID: {}", &id));

        Ok(ReleaseMS::fetch()
            .id(id)
            .with_artists()
            .with_artist_credits()
            .with_release_groups()
            .with_aliases()
            .with_annotations()
            .with_artist_relations()
            .with_recordings()
            .with_recording_level_relations()
            //.with_work_level_relations() https://github.com/oknozor/musicbrainz_rs/pull/87
            //.with_work_relations()
            .execute()
            .await
            .context("Failed to fetch release from MusicBrainz")?
            .into_entity())
    }

    fn get_cache() -> Arc<MusicbrainzCache<Self>> {
        MUSICBRAINZ_DATABASE.releases().clone()
    }

    fn get_mbid(&self) -> PrimaryMBID<Self> {
        PrimaryMBID::from(self.id.to_string().clone())
    }

    fn into_any(self: Arc<Self>) -> AnyMusicBrainzEntity {
        AnyMusicBrainzEntity::Release(self)
    }

    fn try_from_any(value: &AnyMusicBrainzEntity) -> Result<Arc<Self>, Error> {
        match value {
            AnyMusicBrainzEntity::Release(val) => Ok(val.clone()),
            _ => Err(Error::InvalidTypeConvertion(
                "AnyMusicBrainzEntity".to_string(),
                "Release".to_string(),
            )),
        }
    }

    fn incremental_update(self, newer: Self) -> Self {
        Self {
            annotation: newer.annotation.or(self.annotation),
            barcode: newer.barcode.or(self.barcode),
            country: newer.country.or(self.country),
            disambiguation: newer.disambiguation.or(self.disambiguation),
            media: newer.media.or(self.media),
            packaging_id: newer.packaging_id.or(self.packaging_id),
            status_id: newer.status_id.or(self.status_id),
            title: newer.title,
            id: newer.id,
            artist_credit: newer.artist_credit.or(self.artist_credit),
            release_group: newer.release_group.or(self.release_group),
            relations: newer.relations.or(self.relations),
            aliases: newer.aliases.or(self.aliases),
            date: newer.date.or(self.date),
            genres: newer.genres.or(self.genres),
            packaging: newer.packaging.or(self.packaging),
            //quality: newer.quality.or(self.quality),
            status: newer.status.or(self.status),
            tags: newer.tags.or(self.tags),
        }
    }

    fn get_kind() -> crate::models::data::musicbrainz::entity::entity_kind::MusicbrainzEntityKind {
        crate::models::data::musicbrainz::entity::entity_kind::MusicbrainzEntityKind::Release
    }
}
