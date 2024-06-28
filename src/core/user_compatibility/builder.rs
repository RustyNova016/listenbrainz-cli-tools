use color_eyre::eyre::Context;
use derive_getters::Getters;
use tokio::sync::OnceCell;

use crate::models::data::listenbrainz::listen::collection::ListenCollection;
use crate::models::data::listenbrainz::user_listens::UserListens;

use super::UserCompatibility;

#[derive(Getters)]
pub struct UserCompatibilityBuilder {
    user_a: String,
    user_b: String,

    user_a_listens: Option<ListenCollection>,
    user_b_listens: Option<ListenCollection>,
}

impl UserCompatibilityBuilder {
    pub fn new(user_a: String, user_b: String) -> Self {
        Self {
            user_a,
            user_b,
            user_a_listens: None,
            user_b_listens: None,
        }
    }

    pub async fn build(self) -> color_eyre::Result<UserCompatibility> {
        let user_a_listens = self.build_user_a_listens().await?;
        let user_b_listens = self.build_user_b_listens().await?;

        Ok(UserCompatibility::new(
            self.user_a,
            user_a_listens,
            self.user_b,
            user_b_listens,
            OnceCell::new(),
        ))
    }

    pub async fn build_user_a_listens(&self) -> color_eyre::Result<ListenCollection> {
        Ok(match &self.user_a_listens {
            None => UserListens::get_user_with_refresh(&self.user_a)
                .await
                .context("Couldn't fetch the new listens")?
                .get_mapped_listens(),
            Some(val) => val.clone(),
        })
    }

    pub async fn build_user_b_listens(&self) -> color_eyre::Result<ListenCollection> {
        Ok(match &self.user_b_listens {
            None => UserListens::get_user_with_refresh(&self.user_b)
                .await
                .context("Couldn't fetch the new listens")?
                .get_mapped_listens(),
            Some(val) => val.clone(),
        })
    }
}
