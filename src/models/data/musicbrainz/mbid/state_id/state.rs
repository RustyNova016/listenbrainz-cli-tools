use super::MBIDState;
use super::MBIDWithState;

/// This state represent an naive id
pub struct NaiveIDState {}

/// This state represent an primary id
pub struct PrimaryIDState {}

impl MBIDState for NaiveIDState {}
impl MBIDState for PrimaryIDState {}

pub type NaiveMBID<T> = MBIDWithState<T, NaiveIDState>;
pub type PrimaryMBID<T> = MBIDWithState<T, PrimaryIDState>;
