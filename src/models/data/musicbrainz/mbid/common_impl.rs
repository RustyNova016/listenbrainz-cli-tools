use std::marker::PhantomData;
use std::sync::Arc;

use crate::models::data::musicbrainz::entity::is_musicbrainz_entity::IsMusicbrainzEntity;

use super::generic_mbid::IdAliasState;
use super::generic_mbid::MBIDSpe;
use super::generic_mbid::NaiveID;
use super::generic_mbid::NaiveMBID;
use super::is_musicbrainz_id::IsMusicbrainzID;

impl<T, S> MBIDSpe<T, S>
where
    T: IsMusicbrainzEntity,
    S: IdAliasState,
    NaiveMBID<T>: IsMusicbrainzID<T>,
{
    fn into_string(self) -> String {
        self.id
    }

    pub fn as_naive(&self) -> NaiveMBID<T> {
        NaiveMBID::from(self.id().to_string())
    }

    async fn get_entity(&self) -> color_eyre::Result<Arc<T>> {
        T::get_load_or_fetch(&self.clone().as_naive()).await
    }
}
