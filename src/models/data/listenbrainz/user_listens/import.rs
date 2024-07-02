use listenbrainz::raw::response::UserListensListen;

use crate::core::entity_traits::cached::Cached;

use super::UserListens;

impl UserListens {
    /// Import a full data dump
    pub async fn import_dump(
        username: &str,
        listens: Vec<UserListensListen>,
    ) -> color_eyre::Result<()> {
        let mut new_data = Self::new(username);

        for listen in listens {
            new_data.insert_lb_listen(listen);
        }

        //println!("{new_data:#?}");

        Self::get_cache()
            .set(new_data.username(), new_data.clone())
            .await?;

        Ok(())
    }
}
