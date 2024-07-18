use std::sync::Arc;

use futures::TryStreamExt;
use tokio::sync::RwLock;

use crate::models::data::listenbrainz::listen::primary_listen::PrimaryListen;
use crate::models::data::musicbrainz::artist::Artist;
use crate::models::data::musicbrainz::mbid::state_id::state::PrimaryMBID;

use super::ListensWithEntityMap;

impl ListensWithEntityMap<PrimaryMBID<Artist>, Artist> {
    /// Add a listens to its credited artists
    pub async fn add_listen_artist_credits(
        this: &RwLock<Self>,
        listen: Arc<PrimaryListen>,
    ) -> color_eyre::Result<()> {
        let mut data_stream = listen.associate_credited_artist();

        while let Some((entity, listen_ret)) = data_stream.try_next().await? {
            this.write().await.add_listen(entity, listen_ret);
        }

        Ok(())
    }
}
