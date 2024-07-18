use crate::models::data::musicbrainz::entity::traits::MusicBrainzEntity;
use crate::models::data::musicbrainz::mbid::state_id::state::NaiveIDState;
use crate::models::data::musicbrainz::mbid::state_id::MBIDWithState;

impl<T> MBIDWithState<T, NaiveIDState> where T: MusicBrainzEntity {}
