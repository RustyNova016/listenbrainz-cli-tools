use user_compatibility::get_shared_ratio;
use user_compatibility::get_shared_recordings_between_users;
use user_compatibility::get_user_shared_percent;

use crate::database::get_conn;
use crate::database::listenbrainz::listens::ListenFetchQuery;
use crate::database::listenbrainz::listens::ListenFetchQueryReturn;
use crate::datastructures::entity_with_listens::recording_with_listens::RecordingWithListens;
use crate::utils::cli::await_next;

pub mod user_compatibility;

pub async fn compatibility_command(user_a: &str, user_b: &str) {
    let mut conn = get_conn().await;
    let user_a_listens = ListenFetchQuery::builder()
        //.fetch_recordings_redirects(true)
        .returns(ListenFetchQueryReturn::Mapped)
        .user(user_a.to_string())
        .build()
        .fetch(&mut conn)
        .await
        .expect("Couldn't fetch the listens");

    let user_a_recordings = RecordingWithListens::from_listencollection(&mut conn, user_a_listens)
        .await
        .expect("Couldn't get the listened recordings");

    let user_b_listens = ListenFetchQuery::builder()
        //.fetch_recordings_redirects(true)
        .returns(ListenFetchQueryReturn::Mapped)
        .user(user_b.to_string())
        .build()
        .fetch(&mut conn)
        .await
        .expect("Couldn't fetch the listens");

    let user_b_recordings = RecordingWithListens::from_listencollection(&mut conn, user_b_listens)
        .await
        .expect("Couldn't get the listened recordings");

    let shared_recordings =
        get_shared_recordings_between_users(&user_a_recordings, &user_b_recordings);

    println!(
        "
Compatibility results:

[Shared Recordings]
    There is {} recordings both listened by {user_a} and {user_b}
        > This is {}% of {user_a}'s listened recordings
        > This is {}% of {user_b}'s listened recordings

[Compatibility]
    The compatibilty score between {user_a} and {user_b} is {}%
    ",
        shared_recordings.len(),
        get_user_shared_percent(&shared_recordings, &user_a_recordings).trunc_with_scale(2),
        get_user_shared_percent(&shared_recordings, &user_b_recordings).trunc_with_scale(2),
        get_shared_ratio(&shared_recordings, &user_a_recordings, &user_b_recordings)
    );

    await_next();
}

// #[tokio::test]
// #[serial_test::serial]
// async fn compatibility() {
//     compatibility_command("RustyNova", "backhdlp").await;
// }
