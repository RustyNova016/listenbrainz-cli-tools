use chrono::DateTime;
use chrono::Utc;

use crate::models::data::musicbrainz::recording::mbid::RecordingMBID;

pub struct MappedListen {
    recording_id: RecordingMBID,
    user: String,
    listen_at: DateTime<Utc>,
    
} 