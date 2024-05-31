use serde::{Deserialize, Serialize};

pub mod converters;
pub mod getters;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct Track {
    pub recording: String,
    pub title: String,
    pub number: String,
    pub length: Option<u32>,
    pub position: u32,
    pub id: String,
}
