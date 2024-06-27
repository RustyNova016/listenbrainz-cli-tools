use crate::models::data::musicbrainz::entity::is_musicbrainz_entity::IsMusicbrainzEntity;
use crate::models::data::musicbrainz::external_musicbrainz_entity::ExternalMusicBrainzEntity;
use std::future::Future;
use std::hash::Hash;

pub trait IsMusicbrainzID<T: IsMusicbrainzEntity>
where
    Self: Hash + Eq,
{
    fn fetch(
        &self,
    ) -> impl Future<Output = color_eyre::Result<ExternalMusicBrainzEntity>> + Send + Sync;
}
