use chrono::Duration;
use derive_getters::Getters;
use derive_new::new;

#[derive(Debug, Clone, Getters, PartialEq, Eq, new)]
pub struct ListenRate {
    recording: String, // TODO: Convert to RecordingMBID
    listen_count: u64,
    duration: Duration,
}
