use color_eyre::eyre::Context;
use listenbrainz::raw;

use crate::utils::ListenAPIPaginatorBuilder;

use super::data::listens::{collection::UserListenCollection, UserListen};

pub fn fetch_listens(username: &str) -> color_eyre::Result<UserListenCollection> {
    let mut builder = ListenAPIPaginatorBuilder::default();
    builder.user_name(username);

    let reader = builder
        .build()
        .context("Couldn't create ListenReader")?
        .into_reader();

    let mut listen_coll = UserListenCollection::new();
    for raw_listen in reader.into_iter() {
        listen_coll.push(UserListen::try_from(raw_listen).unwrap()); //TODO: Remove ugly unwrap
    }

    Ok(listen_coll)
}
