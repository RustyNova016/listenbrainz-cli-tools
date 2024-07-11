use crate::models::data::musicbrainz::entity::entity_kind::MusicbrainzEntityKind;
use crate::models::data::musicbrainz::entity::traits::MusicBrainzEntity;
use crate::models::data::musicbrainz::mbid::state_id::any::any_entity::AnyEntityMBID;
use crate::models::data::musicbrainz::mbid::state_id::MBIDState;
use crate::models::data::musicbrainz::mbid::state_id::MBIDWithState;

impl<T, S> From<MBIDWithState<T, S>> for AnyEntityMBID<S>
where
    T: MusicBrainzEntity,
    S: MBIDState,
{
    fn from(value: MBIDWithState<T, S>) -> Self {
        let kind = T::get_kind();

        match kind {
            MusicbrainzEntityKind::Artist => Self::Artist(MBIDWithState::from(value.id)),
            MusicbrainzEntityKind::Recording => Self::Recording(MBIDWithState::from(value.id)),
            MusicbrainzEntityKind::Release => Self::Release(MBIDWithState::from(value.id)),
            MusicbrainzEntityKind::ReleaseGroup => {
                Self::ReleaseGroup(MBIDWithState::from(value.id))
            }
            MusicbrainzEntityKind::Work => Self::Work(MBIDWithState::from(value.id)),
            _ => todo!(),
        }
    }
}
