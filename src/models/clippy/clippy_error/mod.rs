pub mod missing_work;
pub mod release;

use std::fmt::format;
use std::fmt::Display;

use color_eyre::owo_colors::OwoColorize;
use itertools::Itertools;

use crate::models::data::musicbrainz::mbid::MBID;
use crate::models::data::musicbrainz::recording::mbid::RecordingMBID;

pub enum ClippyError {
    MissingWork,
}


pub trait IsClippyError: Sized {
    async fn check_for_error(id: MBID) -> color_eyre::Result<Option<Self>>;

    fn get_title(&self) -> String;

    fn get_relevant_url(&self) -> String;

    fn get_description(&self) -> String;

    fn get_additions(&self) -> Vec<(String, String)>;

    fn to_formated(&self) -> String {
        let line1 = format!("{} {}", "Warning:".yellow().bold(), self.get_title().bold());
        let line2 = format!("{} {}", "  --> ".cyan().bold(), self.get_relevant_url());

        let mut description_indent = "   | ".to_string().cyan().to_string();
        description_indent = description_indent.bold().to_string();
        let description_content: String = self.get_description().split("\n").map(|string| format!("{}{}\n",description_indent, string)).collect();
        let description = format!("{description_indent}\n{description_content}{description_indent}");

        format!("{line1}\n{line2}\n{description}\n")
    }

    async fn check_and_print(id: MBID)  -> color_eyre::Result<bool> {
        let result  = Self::check_for_error(id).await?;

        if let Some(err) = result {
            println!("{}", err.to_formated());
            return Ok(true);
        }

        Ok(false)
    }
}