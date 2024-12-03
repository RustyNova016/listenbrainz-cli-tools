use chrono::Datelike;
use chrono::TimeZone;
use chrono::Utc;
use color_eyre::owo_colors::OwoColorize;
use itertools::Itertools;

use crate::database::get_conn;
use crate::database::listenbrainz::listens::ListenFetchQuery;
use crate::database::listenbrainz::listens::ListenFetchQueryReturn;
use crate::database::musicbrainz::anniversaries::get_recordings_aniversaries;
use crate::datastructures::entity_with_listens::recording_with_listens::RecordingWithListens;
use crate::datastructures::listen_collection::traits::ListenCollectionLike;
use crate::utils::cli::print_recording;

pub async fn daily_report(username: &str) {
    let mut conn = get_conn().await;
    let listens = ListenFetchQuery::builder()
        //.fetch_recordings_redirects(true)
        .returns(ListenFetchQueryReturn::Mapped)
        .user(username.to_string())
        .build()
        .fetch(&mut conn)
        .await
        .expect("Couldn't fetch the new listens");

    let recordings = RecordingWithListens::from_listencollection(&mut *get_conn().await, listens)
        .await
        .expect("Couldn't get listen's recordings");

    // release days
    let today = Utc::now();
    //let today = Utc.timestamp_opt(1728508067, 0).unwrap();
    let release_day_recordings = get_recordings_aniversaries(&mut conn, &today)
        .await
        .expect("Couldn't get the recording anniversaries");

    let anniversary_recordings = release_day_recordings
        .iter()
        .filter_map(|rec| recordings.get_by_id(rec.id))
        .collect_vec();

    println!();

    if !anniversary_recordings.is_empty() {
        println!("{}", " Today in history 🎂 ".on_green().black().bold());

        for rec in anniversary_recordings {
            println!(
                "   - {} ({}, {} Listens)",
                print_recording(&mut conn, rec.recording())
                    .await
                    .expect("Couldn't get artist credits"),
                Utc.timestamp_opt(rec.recording().first_release_date.unwrap(), 0)
                    .unwrap()
                    .year(),
                rec.listen_count()
            );
        }
    }

    println!();

    let first_discoveries = recordings
        .values()
        .filter(|rec| {
            rec.first_listen_date()
                .is_some_and(|date| date.day() == today.day() && date.month() == today.month())
        })
        .collect_vec();

    if !first_discoveries.is_empty() {
        println!(
            "{}",
            " You discovered those on this day 🔎 "
                .on_green()
                .black()
                .bold()
        );

        for rec in first_discoveries {
            println!(
                "   - {} ({}, {} Listens)",
                print_recording(&mut conn, rec.recording())
                    .await
                    .expect("Couldn't get artist credits"),
                rec.first_listen_date()
                    .expect("There should be at least a listen")
                    .format("%d/%m/%Y"),
                rec.listen_count()
            );
        }
    }

    // LB return *all* the fresh releases and not just the ones that are for {user}. It would be possible to filter them in place but... TODO!
    // let fresh_releases = FreshReleaseRequest::builder().days(7).future(false).release_date(today).build().fetch().await.expect("Couldn't get fresh listens");

    // if !fresh_releases.is_empty() {
    //     println!(
    //         "{}",
    //         " Fresh releases of the past 7 days "
    //             .on_green()
    //             .black()
    //             .bold()
    //     );

    //     for fresh_release in fresh_releases {
    //         let rg = ReleaseGroup::get_or_fetch(&mut conn, &fresh_release.release_group_mbid).await.expect("Couldn't retrieve release group data").expect("Couldn't find the release group");

    //         println!(
    //             "   - {} ",
    //             print_release_group_lb(&mut conn, &rg).await.expect("Couldn't get artist credits")
    //         );
    //     }
    // }
}

// #[cfg(test)]
// mod tests {
//     use crate::tools::daily::daily_report;

//     #[tokio::test]
//     #[serial_test::serial]
//     async fn daily_report_test() {
//         daily_report("RustyNova").await;
//     }
// }
