use crate::models::data::musicbrainz::artist::Artist;
use crate::models::data::musicbrainz::entity::entity_kind::MusicbrainzEntityKind;
use crate::models::data::musicbrainz::mbid::state_id::state::NaiveIDState;
use crate::models::data::musicbrainz::mbid::state_id::MBIDState;
use crate::models::data::musicbrainz::mbid::state_id::MBIDWithState;
use crate::models::data::musicbrainz::recording::Recording;
use crate::models::data::musicbrainz::release::Release;
use crate::models::data::musicbrainz::release_group::ReleaseGroup;
use crate::models::data::musicbrainz::work::Work;
use crate::models::error::Error;
use crate::utils::regex::is_string_mbid;
use crate::utils::regex::parse_mbid_from_url;
use derive_more::Unwrap;

#[derive(Unwrap)]
pub enum AnyEntityMBID<S: MBIDState> {
    Artist(MBIDWithState<Artist, S>),
    Recording(MBIDWithState<Recording, S>),
    Release(MBIDWithState<Release, S>),
    ReleaseGroup(MBIDWithState<ReleaseGroup, S>),
    Work(MBIDWithState<Work, S>),
}

impl<S: MBIDState> AnyEntityMBID<S> {
    pub fn get_kind(&self) -> MusicbrainzEntityKind {
        match self {
            Self::Artist(_) => MusicbrainzEntityKind::Artist,
            Self::Recording(_) => MusicbrainzEntityKind::Recording,
            Self::Release(_) => MusicbrainzEntityKind::Release,
            Self::ReleaseGroup(_) => MusicbrainzEntityKind::ReleaseGroup,
            Self::Work(_) => MusicbrainzEntityKind::Work,
        }
    }

    pub fn from_string(value: String, kind: MusicbrainzEntityKind) -> AnyEntityMBID<NaiveIDState> {
        match kind {
            MusicbrainzEntityKind::Artist => AnyEntityMBID::<NaiveIDState>::Artist(
                MBIDWithState::<Artist, NaiveIDState>::from(value),
            ),
            MusicbrainzEntityKind::Recording => AnyEntityMBID::<NaiveIDState>::Recording(
                MBIDWithState::<Recording, NaiveIDState>::from(value),
            ),
            MusicbrainzEntityKind::Release => AnyEntityMBID::<NaiveIDState>::Release(
                MBIDWithState::<Release, NaiveIDState>::from(value),
            ),
            MusicbrainzEntityKind::ReleaseGroup => {
                AnyEntityMBID::<NaiveIDState>::ReleaseGroup(MBIDWithState::<
                    ReleaseGroup,
                    NaiveIDState,
                >::from(value))
            }
            MusicbrainzEntityKind::Work => {
                AnyEntityMBID::<NaiveIDState>::Work(MBIDWithState::<Work, NaiveIDState>::from(
                    value,
                ))
            }
            _ => todo!(),
        }
    }

    pub fn parse_string(
        value: String,
        kind: MusicbrainzEntityKind,
    ) -> Result<AnyEntityMBID<NaiveIDState>, Error> {
        if is_string_mbid(&value) {
            Ok(Self::from_string(value, kind))
        } else {
            Err(Error::MBIDStringParsingError)
        }
    }

    pub fn parse_url(value: String) -> Result<AnyEntityMBID<NaiveIDState>, Error> {
        parse_mbid_from_url(&value).ok_or(Error::MBIDStringParsingError)
    }

    pub fn parse_string_or_url(
        value: String,
        kind: MusicbrainzEntityKind,
    ) -> Result<AnyEntityMBID<NaiveIDState>, Error> {
        Self::parse_string(value.clone(), kind).or_else(|_| Self::parse_url(value))
    }
}
