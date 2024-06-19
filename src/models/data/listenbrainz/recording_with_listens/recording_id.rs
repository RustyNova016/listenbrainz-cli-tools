use crate::models::data::listenbrainz::listen::collection::common::ListenCollectionCommons;
use crate::models::data::listenbrainz::listen::collection::mapped_listen_collection::MappedListenCollection;
use crate::models::data::listenbrainz::listen::collection::mapped_listen_collection::MappedListenCollectionExt;
use crate::models::data::musicbrainz::mbid::generic_mbid::MBIDSpe;
use crate::models::data::musicbrainz::mbid::generic_mbid::PrimaryID;
use crate::models::data::musicbrainz::recording::Recording;

use chrono::DateTime;
use chrono::Duration;
use chrono::Utc;
use derive_getters::Getters;

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
pub struct RecordingIDWithListens {
    recording_id: MBIDSpe<Recording, PrimaryID>,
    listens: MappedListenCollection,
}

impl RecordingIDWithListens {
    pub fn new(
        recording_id: MBIDSpe<Recording, PrimaryID>,
        listens: MappedListenCollection,
    ) -> Self {
        Self {
            recording_id: recording_id.clone(),
            listens: listens.keep_only_recording(&recording_id),
        }
    }

    pub fn first_listen_date(&self) -> Option<DateTime<Utc>> {
        self.listens
            .oldest_listen()
            .map(|listen| *listen.listened_at())
    }

    pub fn last_listen_date(&self) -> Option<DateTime<Utc>> {
        self.listens
            .latest_listen()
            .map(|listen| *listen.listened_at())
    }

    pub fn listen_count(&self) -> usize {
        self.listens.len()
    }

    /// Return the amount of time this recording having known about
    pub fn known_for(&self) -> Option<Duration> {
        self.first_listen_date()
            .map(|discovery| Utc::now() - discovery)
    }

    pub fn average_duration_between_listens(&self) -> Duration {
        self.known_for()
            .and_then(|dur| dur.checked_div(self.listen_count() as i32))
            // If the recording haven't been listened to, then the average time is zero
            .unwrap_or_else(Duration::zero)
    }

    pub fn estimated_date_of_next_listen(&self) -> Option<DateTime<Utc>> {
        self.last_listen_date()
            .map(|listen_date| listen_date + self.average_duration_between_listens())
    }

    pub fn overdue_by(&self) -> Duration {
        self.estimated_date_of_next_listen()
            .map(|next_listen| Utc::now() - next_listen)
            .unwrap_or_else(Duration::zero)
    }

    pub fn is_listened(&self) -> bool {
        !self.listens.is_empty()
    }

    //pub async fn underated_score_single(&self) -> color_eyre::Result<Decimal> {
    //    Ok(self
    //        .listens()
    //        .get_underrated_recordings()
    //        .await?
    //        .first()
    //        .expect("Recording should have a score")
    //        .0)
    //}
}
