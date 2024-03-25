use derive_builder::Builder;

use crate::models::cache::listen_cache::ListenCache;

pub mod user_listens;

pub struct ListenBrainzAPI {
    listen_cache: ListenCache,
}


#[derive(Debug, Builder)]
pub struct ListenFetchRequest {
    #[builder(setter(into))]
    client: ListenCache,

    #[builder(setter(into))]
    users: Vec<String>,

    #[builder(setter(into, strip_option), default = "true")]
    fetch_new: bool,

    #[builder(setter(into, strip_option), default = "false")]
    refresh_unlinked: bool,

    #[builder(setter(into, strip_option), default = "false")]
    refresh_all: bool,
}

impl ListenFetchRequest {}
