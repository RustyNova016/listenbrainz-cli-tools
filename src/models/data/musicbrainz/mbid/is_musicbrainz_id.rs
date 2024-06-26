use std::future::Future;

use crate::models::data::musicbrainz::entity::is_musicbrainz_entity::IsMusicbrainzEntity;
use crate::models::data::musicbrainz::external_musicbrainz_entity::ExternalMusicBrainzEntity;

pub trait IsMusicbrainzID<T: IsMusicbrainzEntity> {
    fn fetch(
        &self,
    ) -> impl Future<Output = color_eyre::Result<ExternalMusicBrainzEntity>> + Send + Sync;
}
