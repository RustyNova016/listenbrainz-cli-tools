use musicbrainz_db_lite::models::musicbrainz::recording::Recording;

use crate::database::get_conn;

pub async fn mb_clippy() {
    let mut conn = get_conn().await;

    let start_node = Recording::get_or_fetch(&mut *conn, "543bb836-fb00-470a-8a27-25941fe0294c").await.unwrap().unwrap();

    
}