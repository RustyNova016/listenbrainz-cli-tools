use derive_getters::Getters;
use musicbrainz_db_lite::models::musicbrainz::release::Release;

use crate::datastructures::listen_collection::{group_by::GroupByReleaseID, ListenCollection};

use super::{impl_entity_with_listens, recording_with_listens::RecordingWithListens};

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
pub struct ReleaseWithListens {
    release: Release,
    listens: Vec<RecordingWithListens>,
}

impl ReleaseWithListens {
    pub async fn from_listencollection(
        conn: &mut sqlx::SqliteConnection,
        listens: ListenCollection,
    ) -> Result<Vec<Self>, crate::Error> {
        Ok(Self::from_group_by(listens.group_by_release(conn).await?))
    }

    pub fn from_group_by(data: GroupByReleaseID) -> Vec<Self> {
        let mut res = Vec::new();

        for (_, (release, gp)) in data.into_iter() {
            res.push(Self {
                release,
                listens: RecordingWithListens::from_group_by(gp),
            });
        }

        res
    }
}

impl_entity_with_listens!(ReleaseWithListens);
