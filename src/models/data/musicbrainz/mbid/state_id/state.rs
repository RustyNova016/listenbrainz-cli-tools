use serde::Deserialize;
use serde::Serialize;

use super::MBIDState;
use super::MBIDWithState;

/// This state represent an naive id
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Hash, Clone)]
pub struct NaiveIDState {}

/// This state represent an primary id
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Hash, Clone)]
pub struct PrimaryIDState {}

impl MBIDState for NaiveIDState {}
impl MBIDState for PrimaryIDState {}

pub type NaiveMBID<T> = MBIDWithState<T, NaiveIDState>;
pub type PrimaryMBID<T> = MBIDWithState<T, PrimaryIDState>;
