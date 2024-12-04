use std::collections::HashMap;

use derive_getters::Getters;
use itertools::Itertools;
use musicbrainz_db_lite::models::listenbrainz::listen::Listen;
use musicbrainz_db_lite::models::musicbrainz::artist::Artist;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;
use musicbrainz_db_lite::RowId;

use crate::datastructures::listen_collection::traits::ListenCollectionLike;
use crate::datastructures::listen_collection::ListenCollection;

use super::recording_with_listens::RecordingWithListens;

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
pub struct ArtistWithListens {
    artist: Artist,
    listens: Vec<RecordingWithListens>,
}

impl ArtistWithListens {
    pub async fn from_listencollection(
        conn: &mut sqlx::SqliteConnection,
        listens: ListenCollection,
    ) -> Result<HashMap<i64, Self>, crate::ErrorKind> {
        // Convert Recordings
        let recordings = RecordingWithListens::from_listencollection(conn, listens).await?;

        let recording_refs = recordings.iter_recordings().collect_vec();

        // Load artists
        let results = Recording::get_artist_from_credits_as_batch(conn, &recording_refs).await?;

        // Convert artists
        let mut out = HashMap::new();

        for (_, (recording, artists)) in results {
            for artist in artists {
                out.entry(artist.get_row_id())
                    .or_insert_with(|| Self {
                        artist,
                        listens: Vec::new(),
                    })
                    .push(recordings.0.get(&recording.id).expect("The artist has been fetched from the recording, so it should be there").clone());
            }
        }

        Ok(out)
    }

    pub fn push(&mut self, value: RecordingWithListens) {
        self.listens.push(value);
    }
}

//impl_entity_with_listens!(ReleaseWithListens);

impl ListenCollectionLike for ArtistWithListens {
    fn iter_listens(&self) -> impl Iterator<Item = &Listen> {
        self.listens.iter().flat_map(|l| l.iter_listens())
    }
}
