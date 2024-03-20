use color_eyre::owo_colors::OwoColorize;
use musicbrainz_rs::{entity::recording::{Recording, RecordingSearchQuery}, Fetch};
use cached::proc_macro::cached;

#[cached]
pub fn get_recording_data(mbid: String) -> Recording {
    println!("{} Getting data for MBID: {}", "[MusicBrainz]".bright_magenta(), &mbid);
    Recording::fetch().id(&mbid).execute().unwrap() //TODO: Handle error
}