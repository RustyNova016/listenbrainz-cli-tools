use itertools::Itertools;
use macon::Builder;
use musicbrainz_db_lite::models::listenbrainz::listen::Listen;

use crate::database::listenbrainz::listens::fetch_latest_listens_of_user;
use crate::datastructures::entity_with_listens::recording_with_listens::RecordingWithListens;
use crate::datastructures::listen_collection::ListenCollection;

/// A querry to generate a list of seed recording using the user's listens
#[derive(Debug, Builder)]
pub struct ListenSeeder {
    #[builder(Default=!)]
    username: String,
}

impl ListenSeeder {
    pub async fn seed(
        self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<Vec<RecordingWithListens>, crate::Error> {
        // Get the listens
        fetch_latest_listens_of_user(conn, &self.username).await?;

        let listens: ListenCollection = sqlx::query_as!(
            Listen,
            "
            SELECT 
                listens.*
            FROM       
                users 
                INNER JOIN listens ON users.name = listens.user 
                INNER JOIN msid_mapping ON listens.recording_msid = msid_mapping.recording_msid
            WHERE
                -- Only for this user
                LOWER(listens.user) = LOWER(?)  
    
                -- Keep only mapped listens 
                AND msid_mapping.user = users.id 
            ORDER BY msid_mapping.recording_mbid",
            self.username
        )
        .fetch_all(&mut *conn)
        .await?
        .into();

        Ok(RecordingWithListens::from_listencollection(conn, listens)
            .await?
            .0
            .into_values()
            .collect_vec())
    }
}
