use std::collections::VecDeque;

use color_eyre::owo_colors::OwoColorize;
use futures::TryStreamExt;
use musicbrainz_db_lite::models::musicbrainz::{main_entities::MainEntity, recording::Recording};

use crate::utils::cli::await_next;
use crate::utils::println_cli;
use crate::{
    database::get_conn, datastructures::clippy::missing_work::MissingWorkLint,
    models::clippy::MbClippyLint,
};

pub async fn mb_clippy(start_mbid: &str) {
    let mut conn = get_conn().await;

    let start_node = Recording::fetch_and_save(&mut conn,start_mbid)
        .await
        .unwrap()
        .expect("Couldn't find MBID");

    let mut queue = VecDeque::new();
    queue.push_back(MainEntity::Recording(start_node));
    let mut seen = Vec::new();

    while let Some(entity) = queue.pop_back() {
        if seen.iter().any(|done: &MainEntity| done.is_equal_by_mbid(&entity)) {
            continue;
        }

        check_lint::<MissingWorkLint>(&mut conn, &entity).await;

        queue.extend(
            get_new_nodes(&mut conn, &entity)
                .await
                .expect("Couldn't get new items to process")
                .into_iter(),
        );
        seen.push(entity);
    }

    println!("No more data to process");
}

async fn check_lint<L: MbClippyLint>(conn: &mut sqlx::SqliteConnection, entity: &MainEntity) {
    let Some(lint) = L::check(conn, entity)
        .await
        .expect("Error while processing lint")
    else {
        return;
    };

    println!("{}", L::get_name().on_yellow().black());
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

    await_next();
}

async fn get_new_nodes(
    conn: &mut sqlx::SqliteConnection,
    entity: &MainEntity,
) -> Result<Vec<MainEntity>, crate::Error> {
    let mut out = Vec::new();

    println_cli("Getting new data...");

    match entity {
        MainEntity::Recording(val) => {
            let artists = val.get_artists_or_fetch(conn).await?;
            for artist in artists {
                out.push(MainEntity::Artist(artist));
            }

            let releases = val.get_releases_or_fetch(conn).await?;
            for release in releases {
                out.push(MainEntity::Release(release));
            }
        }
        MainEntity::Release(val) => {
            let recordings = val.get_recordings_or_fetch(conn).await?;
            for recording in recordings {
                out.push(MainEntity::Recording(recording));
            }
        }
        MainEntity::Artist(val) => {
            let recordings: Vec<Recording> = val
                .browse_or_fetch_artist_recordings(conn)
                .try_collect()
                .await?;
            for recording in recordings {
                out.push(MainEntity::Recording(recording));
            }
        }
        _ => {}
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use crate::tools::musicbrainz::clippy::mb_clippy;

    #[tokio::test]
    async fn mb_clippy_test() {
        mb_clippy("543bb836-fb00-470a-8a27-25941fe0294c").await;
    }
}
