pub mod display;
use core::fmt;
use core::fmt::Display;
use std::fmt::Write as _;
use std::io;

use clap::CommandFactory as _;
use color_eyre::owo_colors::OwoColorize as _;
use musicbrainz_db_lite::models::musicbrainz::artist_credit::ArtistCredits;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;
use musicbrainz_db_lite::models::musicbrainz::release_group::ReleaseGroup;

use super::regex::is_string_mbid;
use crate::models::cli::Cli;
use crate::utils::regex::get_raw_mbid_from_url;

pub mod progress_bar;

pub mod parsing;

/// Block the current trhead until the user press enter
pub fn await_next() {
    let buf = &mut String::new();
    let _ = io::stdin().read_line(buf);
}

pub fn read_mbid_from_input(input: &str) -> Option<String> {
    if is_string_mbid(input) {
        return Some(input.to_string());
    }

    get_raw_mbid_from_url(input)
}

pub fn hyperlink_rename(text: &impl Display, link: &str) -> String {
    let osc8: &str = "\x1b]8";
    let st: &str = "\x1b\\";
    format!(r"{osc8};;{link}{st}{text}{osc8};;{st}")
}

pub fn title_with_credits(title: &str, title_link: &str, credits: &ArtistCredits) -> String {
    let mut out = hyperlink_rename(&title.bright_blue(), title_link);
    write!(out, " by ").unwrap();

    for credit in &credits.1 {
        write!(
            out,
            "{}{}",
            &hyperlink_rename(
                &credit.name.green(),
                &format!("https://listenbrainz.org/artist/{}", credit.artist_gid)
            ),
            &credit.join_phrase
        )
        .unwrap();
    }

    out
}

pub async fn print_recording(
    conn: &mut sqlx::SqliteConnection,
    recording: &Recording,
) -> Result<String, crate::Error> {
    Ok(title_with_credits(
        &recording.title,
        &format!("https://musicbrainz.org/recording/{}", recording.mbid),
        &recording.get_artist_credits_or_fetch(conn).await?,
    ))
}

pub async fn print_release_group_lb(
    conn: &mut sqlx::SqliteConnection,
    val: &ReleaseGroup,
) -> Result<String, crate::Error> {
    Ok(title_with_credits(
        &val.title,
        &format!("https://listenbrainz.org/album/{}", val.mbid),
        &val.get_artist_credits_or_fetch(conn).await?,
    ))
}

pub fn clap_error(msg: impl fmt::Display, error: clap::error::ErrorKind) -> ! {
    Cli::command().error(error, msg).exit()
}
