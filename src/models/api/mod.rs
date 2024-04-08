use std::sync::Arc;

use color_eyre::eyre::Context;

use crate::models::data::listenbrainz::listen::Listen;
use crate::utils::ListenAPIPaginatorBuilder;

use super::data::listenbrainz::listen::collection::ListenCollection;

pub mod listenbrainz;

pub fn fetch_listens(username: &str) -> color_eyre::Result<ListenCollection> {
    let mut builder = ListenAPIPaginatorBuilder::default();
    builder.user_name(username);

    let reader = builder
        .build()
        .context("Couldn't create ListenReader")?
        .into_reader();

    let mut listen_coll = ListenCollection::new();
    for raw_listen in reader.into_iter() {
        listen_coll.push(Arc::new(Listen::from(raw_listen))); //TODO: Remove ugly unwrap
    }

    Ok(listen_coll)
}
