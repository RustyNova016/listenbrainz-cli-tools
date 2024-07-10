use crate::models::data::musicbrainz::entity::traits::fetch_entity::FetchEntity;
use crate::models::data::musicbrainz::external_musicbrainz_entity::ExternalMusicBrainzEntity;
use crate::models::data::musicbrainz::mbid::state_id::MBIDState;
use crate::models::data::musicbrainz::mbid::state_id::MBIDWithState;
use crate::models::data::musicbrainz::mbid::state_id::MusicBrainzEntity;

impl<T, S> MBIDWithState<T, S>
where
    T: MusicBrainzEntity + FetchEntity + ?Sized,
    S: MBIDState,
{
    pub async fn fetch_entity(&self) -> color_eyre::Result<ExternalMusicBrainzEntity> {
        T::fetch(self).await
    }
}
