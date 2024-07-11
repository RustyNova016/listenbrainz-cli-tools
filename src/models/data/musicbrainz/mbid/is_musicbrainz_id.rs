use std::future::Future;

use crate::models::data::musicbrainz::entity::is_musicbrainz_entity::IsMusicbrainzEntity;
use crate::models::data::musicbrainz::external_musicbrainz_entity::ExternalMusicBrainzEntity;

use super::generic_mbid::NaiveMBID;

pub trait IsMusicbrainzID<T>
where
    T: IsMusicbrainzEntity,
    NaiveMBID<T>: IsMusicbrainzID<T>,
{
    fn fetch(
        &self,
    ) -> impl Future<Output = color_eyre::Result<ExternalMusicBrainzEntity>> + Send + Sync;
}
