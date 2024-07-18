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
use musicbrainz_rs::entity::artist::Artist as ArtistMS;
use musicbrainz_rs::Fetch;

use super::external::ArtistExt;
use super::Artist;

impl MusicBrainzEntity for Artist {
    async fn fetch<S: MBIDState>(
        id: &MBIDWithState<Self, S>,
    ) -> color_eyre::Result<ExternalMusicBrainzEntity> {
        println_mus(format!("Getting data for artist MBID: {}", &id));

        Ok(ArtistMS::fetch()
            .id(id)
            .with_aliases()
            .with_artist_relations()
            .with_recording_relations()
            .execute()
            .await
            .context("Failed to fetch artist from MusicBrainz")?
            .into_entity())
    }

    fn get_cache() -> Arc<MusicbrainzCache<Self>> {
        MUSICBRAINZ_DATABASE.artists().clone()
    }

    fn get_mbid(&self) -> PrimaryMBID<Self> {
        PrimaryMBID::from(self.id.to_string().clone())
    }

    fn into_any(self: Arc<Self>) -> AnyMusicBrainzEntity {
        AnyMusicBrainzEntity::Artist(self)
    }

    fn try_from_any(value: &AnyMusicBrainzEntity) -> Result<Arc<Self>, Error> {
        match value {
            AnyMusicBrainzEntity::Artist(val) => Ok(val.clone()),
            _ => Err(Error::InvalidTypeConvertion(
                "AnyMusicBrainzEntity".to_string(),
                "Artist".to_string(),
            )),
        }
    }

    fn incremental_update(self, newer: Self) -> Self {
        Self {
            id: newer.id,
            name: newer.name,
            annotation: newer.annotation.or(self.annotation),
            tags: newer.tags.or(self.tags),
            aliases: newer.aliases.or(self.aliases),
            artist_type: newer.artist_type.or(self.artist_type),
            country: newer.country.or(self.country),
            gender: newer.gender.or(self.gender),
            genres: newer.genres.or(self.genres),
            life_span: newer.life_span.or(self.life_span),
            disambiguation: newer.disambiguation,
            recordings: newer.recordings.or(self.recordings),
            release_groups: newer.release_groups.or(self.release_groups),
            releases: newer.releases.or(self.releases),
            sort_name: newer.sort_name,
            works: newer.works.or(self.works),
            relations: newer.relations.or(self.relations),
        }
    }

    fn get_kind() -> MusicbrainzEntityKind {
        MusicbrainzEntityKind::Artist
    }
}
