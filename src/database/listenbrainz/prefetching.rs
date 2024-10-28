use musicbrainz_db_lite::models::listenbrainz::listen::Listen;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;

use crate::utils::cli::global_progress_bar::PG_FETCHING;

/// Prefetch all the recordings of a list of listens
pub async fn prefetch_recordings_of_listens(
    conn: &mut sqlx::SqliteConnection,
    user_id: i64,
    listens: &[Listen],
) -> Result<(), musicbrainz_db_lite::Error> {
    let recordings = Listen::get_unfetched_recordings_ids(conn, user_id, listens).await?;
    let progress_bar = PG_FETCHING.get_submitter(listens.len() as u64);

    for recording in recordings {
        Recording::fetch_and_save(conn, &recording).await?;
        progress_bar.inc(1);
    }

    Ok(())
}

/// Prefetch all the release of a list of recordings
pub async fn prefetch_releases(
    conn: &mut sqlx::SqliteConnection,
    recordings: &[&Recording] ,
) -> Result<(), musicbrainz_db_lite::Error> {
    let progress_bar = PG_FETCHING.get_submitter(recordings.len() as u64);

    for recording in recordings {
        recording.fetch_if_incomplete(conn).await?;
        progress_bar.inc(1);
    }

    Ok(())
}
