use musicbrainz_rs::{entity::recording::{Recording, RecordingSearchQuery}, Fetch};
use cached::proc_macro::cached;

#[cached]
fn get_recording_data(mbid: String) -> Recording {
    Recording::fetch().id(&mbid).execute().unwrap() //TODO: Handle error
}