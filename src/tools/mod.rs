use color_eyre::eyre::Context;
use listenbrainz::raw::{response::UserListensListen, Client};
use listenbrainz_utils::readers::ListenReaderBuilder;

/// Fetch an user's listens and extract the unlinked ones
pub fn get_all_unlinked_of_user(username: &str) -> Vec<UserListensListen> {
    let client = Client::new();

    let mut builder = ListenReaderBuilder::default();
    builder.user_name(username.to_string());
    let mut reader = builder.build().context("Couldn't create ListenReader").unwrap();

    let mut unlinked = vec![];
    let mut i = 1;
    loop {
        println!("Page: {}", i);
        i += 1;
        let page = reader.next(&client).unwrap();

        for listen in page.payload.listens.into_iter() {
            if listen.track_metadata.mbid_mapping.is_none() {
                unlinked.push(listen);
            }
        }

        if page.payload.count <= 1 {
            break;
        }
    }
    unlinked
}
