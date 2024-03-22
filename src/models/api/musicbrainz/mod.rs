use cached::proc_macro::cached;
use color_eyre::owo_colors::OwoColorize;
use musicbrainz_rs::{
    entity::recording::{Recording, RecordingSearchQuery},
    Fetch,
};

#[cached]
pub fn get_recording_data(mbid: String) -> Recording {
    println!(
        "{} Getting data for recording MBID: {}",
        "[MusicBrainz]".bright_magenta(),
        &mbid
    );
    Recording::fetch().id(&mbid).execute().unwrap() //TODO: Handle error
}

