use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

use musicbrainz_db_lite::models::listenbrainz::listen::Listen;
use musicbrainz_db_lite::models::listenbrainz::messybrainz_submission::MessybrainzSubmission;
use musicbrainz_db_lite::models::listenbrainz::msid_mapping::MsidMapping;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;
use musicbrainz_db_lite::models::musicbrainz::user::User;
use serde::Deserialize;
use serde::Serialize;
use sqlx::Acquire;

use crate::utils::println_cli_info;

pub async fn import_listen_dump(
    conn: &mut sqlx::SqliteConnection,
    dump_path: &Path,
    username: &str,
) {
    let zip_file = File::open(dump_path).expect("Couldn't access zip file.");
    let mut archive = zip::ZipArchive::new(zip_file).expect("Couldn't read zip file.");

    // We read the zip file
    for i in 0..archive.len() {
        let file = archive.by_index(i).unwrap();

        let outpath = match file.enclosed_name() {
            Some(path) => path,
            None => continue,
        };

        // The file is a directory? Skip. We don't need to handle those
        if file.is_dir() {
            continue;
        }

        // The file is actually a listen?
        if outpath.to_string_lossy() == "feedback.jsonl"
            || outpath.to_string_lossy() == "pinned_recording.jsonl"
            || outpath.to_string_lossy() == "user.json"
        {
            continue;
        }

        println!("Saving {}", outpath.display());

        // Convert jsonl to json
        let content = BufReader::new(file).lines();

        // Then save the content
        let mut count = 0;
        let mut trans = conn.begin().await.expect("Couldn't start transaction");
        for line in content {
            let line = line.expect("Couldn't read line");
            //println!("{line}");
            let data: ImportListen =
                serde_json::from_str(&line).expect("Couldn't convert line from JSON");

            data.save(&mut trans, username)
                .await
                .expect("Couldn't save listen");
            count += 1;
        }
        trans.commit().await.expect("Couldn't save transaction");

        println_cli_info(format!("Loaded {count} listens"));
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct ImportListen {
    listened_at: i64,
    track_metadata: ImportListenMetaData,
}

#[derive(Debug, Deserialize, Serialize)]
struct ImportListenMetaData {
    track_name: String,
    artist_name: String,
    release_name: Option<String>,
    recording_msid: String,
    additional_info: HashMap<String, serde_json::Value>,
}

impl ImportListen {
    pub async fn save(
        self,
        conn: &mut sqlx::SqliteConnection,
        user_name: &str,
    ) -> Result<(), crate::Error> {
        // First, get the user
        User::insert_or_ignore(&mut *conn, user_name).await.unwrap();

        let data = serde_json::to_string(&self.track_metadata.additional_info)
            .expect("Crashing from serializing a serde::Value isn't possible");

        let messybrainz = MessybrainzSubmission {
            id: 0,
            msid: self.track_metadata.recording_msid.clone(),
            recording: self.track_metadata.track_name,
            artist_credit: self.track_metadata.artist_name,
            release: self.track_metadata.release_name,
            track_number: None, // TODO: Find where is it stored in the json... If it even is stored...
            duration: None, //TODO: Get the duration from additiona info or ditch it from the schema?
        };

        messybrainz.insert_or_ignore(&mut *conn).await.unwrap();

        // Check if we have a recording MBID. If so, we can map it
        let additional_info = &self.track_metadata.additional_info;

        if let Some(serde_json::Value::String(recording)) = additional_info.get("recording_mbid") {
            // First insert the mbid
            Recording::add_redirect_mbid(conn, recording).await.unwrap();

            let user = User::find_by_name(&mut *conn, user_name)
                .await?
                .expect("The user shall be inserted");

            MsidMapping::set_user_mapping(
                &mut *conn,
                user.id,
                self.track_metadata.recording_msid.clone(),
                recording.to_string(),
            )
            .await
            .unwrap();
        }

        let listen = Listen {
            id: 0,
            listened_at: self.listened_at,
            user: user_name.to_string(),
            recording_msid: self.track_metadata.recording_msid.clone(),
            data: Some(data),
        };

        listen.upsert_listen(conn).await.unwrap();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    // use std::path::PathBuf;
    // use musicbrainz_db_lite::models::listenbrainz::listen::Listen;
    // use crate::database::get_conn;
    // use crate::tools::listens::import::import_listen_dump;

    // #[tokio::test]
    // async fn load_listen_dump_test() {
    //     import_listen_dump(
    //         &PathBuf::from("tests/data/listen_dump.zip".to_string()),
    //         "TestNova",
    //     )
    //     .await;
    //
    //     let conn = &mut *get_conn().await;
    //
    //     let listen = sqlx::query_as!(Listen, "SELECT * FROM listens WHERE listened_at = 1705054374").fetch_one(&mut *conn).await.expect("This listen should exist");
    //     listen.get_recording_or_fetch(conn).await.expect("The listen should be mapped");
    // }
}
