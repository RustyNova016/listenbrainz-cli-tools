use crate::models::data::musicbrainz::artist::Artist;
use crate::models::data::musicbrainz::mbid::state_id::MBIDState;
use crate::models::data::musicbrainz::mbid::state_id::MBIDWithState;
use crate::models::data::musicbrainz::recording::Recording;
use crate::models::data::musicbrainz::release::Release;
use crate::models::data::musicbrainz::release_group::ReleaseGroup;
use crate::models::data::musicbrainz::work::Work;

pub enum AnyEntityMBID<S: MBIDState> {
    Artist(MBIDWithState<Artist, S>),
    Recording(MBIDWithState<Recording, S>),
    Release(MBIDWithState<Release, S>),
    ReleaseGroup(MBIDWithState<ReleaseGroup, S>),
    Work(MBIDWithState<Work, S>),
}

impl<S: MBIDState> AnyEntityMBID<S> {}
