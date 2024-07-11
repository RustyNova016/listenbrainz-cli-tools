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
            MusicbrainzEntityKind::Artist => AnyEntityMBID::Artist(MBIDWithState::from(value.id)),
            MusicbrainzEntityKind::Recording => {
                AnyEntityMBID::Recording(MBIDWithState::from(value.id))
            }
            MusicbrainzEntityKind::Release => AnyEntityMBID::Release(MBIDWithState::from(value.id)),
            MusicbrainzEntityKind::ReleaseGroup => {
                AnyEntityMBID::ReleaseGroup(MBIDWithState::from(value.id))
            }
            MusicbrainzEntityKind::Work => AnyEntityMBID::Work(MBIDWithState::from(value.id)),
            _ => todo!(),
        }
    }
}
