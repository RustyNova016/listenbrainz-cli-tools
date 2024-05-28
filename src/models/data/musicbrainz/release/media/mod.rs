use serde::{Deserialize, Serialize};

use super::track::Track;

pub mod caching;
pub mod converters;
pub mod getters;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct Media {
    title: Option<String>,
    position: Option<u32>,
    track_count: u32,
    disc_count: Option<u32>,
    format_id: Option<String>,
    format: Option<String>,
    tracks: Option<Vec<Track>>,
}
