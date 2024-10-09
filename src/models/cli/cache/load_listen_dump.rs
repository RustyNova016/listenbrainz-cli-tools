use std::fs::File;
use std::path::PathBuf;

use listenbrainz::raw::response::UserListensListen;

use crate::models::data::listenbrainz::user_listens::UserListens;
use crate::utils::println_cli_info;

//TODO: Move this file in the tool folder

pub async fn load_listen_dump(
    path: &PathBuf,
    username: &str,
) -> Result<(), color_eyre::eyre::Error> {
    let file = File::open(path)?;
    let listens: Vec<UserListensListen> = serde_json::from_reader(file)?;

    println_cli_info(format!("Loaded {} listens", listens.len()));

    UserListens::import_dump(username, listens).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::models::cli::cache::load_listen_dump::load_listen_dump;
    use crate::models::data::entity_database::ENTITY_DATABASE;

    #[tokio::test]
    async fn load_listen_dump_test() {
        let res = load_listen_dump(
            &PathBuf::from("tests/data/listen_dump.json".to_string()),
            "TestNova",
        )
        .await;
        assert!(res.is_ok());

        let listens = ENTITY_DATABASE
            .user_listens()
            .get("testnova")
            .await
            .unwrap();
        assert!(listens.is_some());

        let listens = listens.unwrap();

        assert_eq!(listens.len(), 12994);
    }
}
