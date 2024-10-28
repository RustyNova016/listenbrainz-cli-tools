pub mod listenbrainz;
use std::fs::{self, File};
use std::path::PathBuf;

use directories::BaseDirs;
use musicbrainz_db_lite::database::client::DBClient;
use once_cell::sync::{Lazy, OnceCell};

static MUSICBRAINZ_LITE: OnceCell<DBClient> = OnceCell::new();

pub static DB_LOCATION: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = BaseDirs::new()
        .expect("Couldn't find standard directory. Is your system an oddball one?")
        .cache_dir()
        .to_path_buf();
    path.push("listenbrainz_cli_tools");
    #[cfg(debug_assertions)]
    path.push("debug/debug_db.db");

    #[cfg(not(debug_assertions))]
    path.push("data.db");

    println!("{}", path.to_str().unwrap());
    if !fs::exists(&path).unwrap() {
        fs::create_dir_all(&path).expect("Couldn't create cache directory");
    }
    path
});

pub async fn get_db_client() -> &'static DBClient {
    if let Some(conn) = MUSICBRAINZ_LITE.get() {
        return conn;
    }

    let client = connect_and_setup().await.unwrap();

    MUSICBRAINZ_LITE.get_or_init(|| client)
}

pub async fn get_conn() -> sqlx::pool::PoolConnection<sqlx::Sqlite> {
    get_db_client()
        .await
        .connection
        .acquire()
        .await
        .expect("Couldn't get connection to the SQLite database")
}

async fn setup_database() -> Result<DBClient, musicbrainz_db_lite::Error> {
    File::create_new(DB_LOCATION.to_str().unwrap()).unwrap();

    let client = DBClient::connect(DB_LOCATION.to_str().unwrap()).await?;
    //client.create_database().await?;

    Ok(client)
}

async fn connect_to_db() -> Result<Option<DBClient>, musicbrainz_db_lite::Error> {
    if std::fs::exists(DB_LOCATION.to_str().unwrap()).unwrap() {
        return Ok(Some(
            DBClient::connect(DB_LOCATION.to_str().unwrap()).await?,
        ));
    }

    Ok(None)
}

async fn connect_and_setup() -> Result<DBClient, musicbrainz_db_lite::Error> {
    match connect_to_db().await? {
        None => todo!(), //setup_database().await,
        Some(client) => {
            //if !check_db_integrity(&client).await.unwrap() {
            //    println!("Remaking Database File for new schema");
            //    drop(client);
            //    fs::remove_file("./tests/test_db.db").unwrap();
            //    return setup_database().await;
            //}

            Ok(client)
        }
    }
}
