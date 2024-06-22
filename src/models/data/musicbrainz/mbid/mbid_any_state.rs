use serde::Deserialize;
use serde::Serialize;

use super::generic_mbid::IdEntityType;
use super::generic_mbid::MBIDSpe;
use super::generic_mbid::NaiveID;
use super::generic_mbid::PrimaryID;
use derive_more::*;

#[derive(Debug, From, Deserialize, Serialize, PartialEq, Eq, Clone, IsVariant, Unwrap)]
pub enum MBIDAnyState<T: IdEntityType> {
    Naive(MBIDSpe<T, NaiveID>),
    Primary(MBIDSpe<T, PrimaryID>),
}
