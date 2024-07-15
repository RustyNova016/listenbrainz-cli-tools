use crate::models::data::listenbrainz::listen::collection::mapped_primary_collection::MappedPrimaryListenCollectionExt;
use crate::models::data::listenbrainz::listen::collection::mapped_primary_collection::PrimaryListenCollection;
use crate::models::data::listenbrainz::listen::collection::traits::CollectionOfListens;
use crate::models::data::musicbrainz::mbid::state_id::state::PrimaryMBID;
use crate::models::data::musicbrainz::recording::Recording;

use chrono::DateTime;
use chrono::Duration;
use chrono::Utc;
use derive_getters::Getters;
use itertools::Itertools;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
pub struct RecordingIDWithListens {
    recording_id: PrimaryMBID<Recording>,
    listens: PrimaryListenCollection,
}

impl RecordingIDWithListens {
    // pub fn new(recording_id: PrimaryMBID<Recording>, listens: PrimaryListenCollection) -> Self {
    //     //TODO: Perf Testing
    //     assert!(
    //         listens.has_only_recording(&recording_id),
    //         "Tried to insert a listen list that contain a listen from another recording"
    //     );

    //     Self {
    //         recording_id,
    //         listens,
    //     }
    // }

    /// Create a new [`RecordingIDWithListens`] from an id and a list of unfiltered listens
    pub fn new_from_unfiltered(
        recording_id: PrimaryMBID<Recording>,
        listens: &PrimaryListenCollection,
    ) -> Self {
        let filtered = listens.get_listens_of_recording(&recording_id);

        Self {
            recording_id,
            listens: filtered,
        }
    }

    pub fn all_from_unfiltered(listens: &PrimaryListenCollection) -> color_eyre::Result<Vec<Self>> {
        let recordings = listens.clone().into_mbids();

        Ok(recordings
            .into_iter()
            .map(|rec| Self::new_from_unfiltered(rec, listens))
            .collect_vec())
    }

    pub fn first_listen_date(&self) -> Option<DateTime<Utc>> {
        self.listens
            .find_oldest_listen()
            .map(|listen| *listen.get_listened_at())
    }

    pub fn last_listen_date(&self) -> Option<DateTime<Utc>> {
        self.listens
            .find_latest_listen()
            .map(|listen| *listen.get_listened_at())
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

    pub async fn underated_score_single(&self) -> color_eyre::Result<Decimal> {
        Ok(self
            .listens
            .clone()
            .into_legacy()
            .get_underrated_recordings()
            .await?
            .first()
            .expect("Recording should have a score")
            .0)
    }

    pub fn overdue_score(&self) -> Decimal {
        Decimal::from_i64(self.overdue_by().num_seconds()).unwrap()
            / Decimal::from_i64(self.average_duration_between_listens().num_seconds()).unwrap()
    }
}
