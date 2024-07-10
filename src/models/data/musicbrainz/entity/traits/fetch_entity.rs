use crate::models::data::musicbrainz::external_musicbrainz_entity::ExternalMusicBrainzEntity;
use crate::models::data::musicbrainz::mbid::state_id::MBIDState;
use crate::models::data::musicbrainz::mbid::state_id::MBIDWithState;
use crate::models::data::musicbrainz::mbid::state_id::MusicBrainzEntity;

pub trait FetchEntity: MusicBrainzEntity {
    async fn fetch<S: MBIDState>(
        id: &MBIDWithState<Self, S>,
    ) -> color_eyre::Result<ExternalMusicBrainzEntity>;
}
