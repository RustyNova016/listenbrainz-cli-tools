use core::cmp::Reverse;

use rust_decimal::Decimal;

use crate::datastructures::entity_with_listens::recording_with_listens::RecordingWithListens;
use crate::models::config::Config;

pub fn overdue_sorter(mut recordings: Vec<RecordingWithListens>) -> Vec<RecordingWithListens> {
    let conf = Config::load_or_panic();

    recordings.sort_by_cached_key(|r| {
        let score = r.overdue_by().num_seconds();
        Decimal::from(score)
            * conf
                .read_or_panic()
                .bumps
                .get_multiplier(&r.recording().mbid)
    });

    recordings
}

pub fn overdue_factor_sorter(
    mut recordings: Vec<RecordingWithListens>,
) -> Vec<RecordingWithListens> {
    let conf = Config::load_or_panic();

    recordings.sort_by_cached_key(|r| {
        let score = r.overdue_factor() + Decimal::ONE;
        Reverse(
            score
                * conf
                    .read_or_panic()
                    .bumps
                    .get_multiplier(&r.recording().mbid),
        )
    });

    recordings
}
