use crate::models::data::musicbrainz::artist::Artist;
use crate::models::data::musicbrainz::recording::Recording;
use crate::models::data::musicbrainz::release::Release;
use crate::models::data::musicbrainz::release_group::ReleaseGroup;
use crate::models::data::musicbrainz::work::Work;

use super::generic_mbid::IdAliasState;
use super::generic_mbid::MBIDSpe;

pub enum AnyMBIDType<S>
where
    S: IdAliasState,
{
    Artist(MBIDSpe<Artist, S>),
    ReleaseGroup(MBIDSpe<ReleaseGroup, S>),
    Release(MBIDSpe<Release, S>),
    Recording(MBIDSpe<Recording, S>),
    Work(MBIDSpe<Work, S>),
}
