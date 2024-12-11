use std::collections::VecDeque;

use color_eyre::owo_colors::OwoColorize;
use futures::TryStreamExt;
use musicbrainz_db_lite::models::musicbrainz::main_entities::MainEntity;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;

use crate::database::get_conn;
use crate::datastructures::clippy::missing_release_barcode::MissingBarcodeLint;
use crate::datastructures::clippy::missing_work::MissingWorkLint;
use crate::models::clippy::MbClippyLint;
use crate::utils::cli::await_next;
use crate::utils::cli::display::MainEntityExt;
use crate::utils::println_cli;

pub async fn mb_clippy(start_mbid: &str, new_first: bool) {
    let mut conn = get_conn().await;

    let start_node = Recording::fetch_and_save(&mut conn, start_mbid)
        .await
        .unwrap()
        .expect("Couldn't find MBID");

    let mut queue = VecDeque::new();
    queue.push_back(MainEntity::Recording(start_node));
    let mut seen = Vec::new();

    while let Some(mut entity) = get_new_element(&mut queue, new_first) {
        if seen
            .iter()
            .any(|done: &MainEntity| done.is_equal_by_mbid(&entity))
        {
            continue;
        }

        entity
            .refetch_and_load(&mut conn)
            .await
            .expect("Couldn't fetch entity");

        check_lint::<MissingWorkLint>(&mut conn, &mut entity).await;
        check_lint::<MissingBarcodeLint>(&mut conn, &mut entity).await;

        println!(
            "Checked {}",
            entity
                .pretty_format(&mut conn, false)
                .await
                .expect("Error while formating the name of the entity")
        );
        println!();

        get_new_nodes(&mut conn, &entity, &mut queue)
            .await
            .expect("Couldn't get new items to process");

        seen.push(entity);
    }

    println!("No more data to process");
}

fn get_new_element(queue: &mut VecDeque<MainEntity>, new_first: bool) -> Option<MainEntity> {
    if new_first {
        queue.pop_front()
    } else {
        queue.pop_back()
    }
}

async fn check_lint<L: MbClippyLint>(conn: &mut sqlx::SqliteConnection, entity: &mut MainEntity) {
    let Some(lint) = L::check(conn, entity)
        .await
        .expect("Error while processing lint")
    else {
        return;
    };

    println!("{}", format!("\n {} ", L::get_name()).on_yellow().black());
    println!();
    println!(
        "{}",
        lint.get_body(conn)
            .await
            .expect("Error while processing lint body")
    );
    println!();
    println!("Links:");
    for link in lint
        .get_links(conn)
        .await
        .expect("Error while processing lint links")
    {
        println!("    - {link}");
    }

    println!();
    await_next();
    entity
        .refetch_and_load(conn)
        .await
        .expect("Couldn't fetch entity");
}

async fn get_new_nodes(
    conn: &mut sqlx::SqliteConnection,
    entity: &MainEntity,
    queue: &mut VecDeque<MainEntity>,
) -> Result<(), crate::Error> {
    println_cli("Getting new data...");

    match entity {
        MainEntity::Recording(val) => {
            let artists = val.get_artists_or_fetch(conn).await?;
            for artist in artists {
                queue.push_front(MainEntity::Artist(artist));
            }

            let releases = val.get_releases_or_fetch(conn).await?;
            for release in releases {
                queue.push_front(MainEntity::Release(release));
            }

            let works = val.get_works_or_fetch(conn).await?;
            for work in works {
                queue.push_front(MainEntity::Work(work));
            }
        }
        MainEntity::Release(val) => {
            let recordings = val.get_recordings_or_fetch(conn).await?;
            for recording in recordings {
                queue.push_front(MainEntity::Recording(recording));
            }
        }
        MainEntity::Artist(val) => {
            let recordings: Vec<Recording> = val
                .browse_or_fetch_artist_recordings(conn)
                .try_collect()
                .await?;
            for recording in recordings {
                queue.push_front(MainEntity::Recording(recording));
            }
        }
        _ => {}
    }

    Ok(())
}

// #[cfg(test)]
// mod tests {
//     use crate::tools::musicbrainz::clippy::mb_clippy;

//     #[tokio::test]
//     async fn mb_clippy_test() {
//         mb_clippy("543bb836-fb00-470a-8a27-25941fe0294c", false).await;
//     }
// }