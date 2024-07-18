use crate::models::data::musicbrainz::entity::traits::MusicBrainzEntity;
use crate::models::data::musicbrainz::mbid::state_id::state::NaiveMBID;
use crate::models::data::musicbrainz::mbid::state_id::state::PrimaryMBID;

impl<T> NaiveMBID<T>
where
    T: MusicBrainzEntity,
{
    pub async fn into_primary(self) -> color_eyre::Result<PrimaryMBID<T>> {
        Ok(self.get_load_or_fetch().await?.get_mbid())
    }
}
