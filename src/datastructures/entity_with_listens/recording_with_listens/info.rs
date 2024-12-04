use chrono::Duration;
use chrono::Local;
use color_eyre::owo_colors::OwoColorize;
use humantime::format_duration;
use indoc::formatdoc;
use rust_decimal::Decimal;

use crate::datastructures::listen_collection::traits::ListenCollectionLike;
use crate::models::config::Config;
use crate::models::data::musicbrainz::recording::mbid::RecordingMBID;
use crate::utils::extensions::chrono_ext::DateTimeUtcExt;
use crate::utils::extensions::chrono_ext::DurationExt;

use super::RecordingWithListens;

impl RecordingWithListens {
    /// Generate a formated string with all the informations to display in reports
    pub fn get_lookup_report(&self) -> String {
        if self.is_empty() {
            return self.generate_empty_report(&self.recording.title);
        }

        self.generate_full_report(&self.recording.title)
    }

    /// Generate a formated string with all the informations to display in reports
    pub async fn get_lookup_report_async(
        &self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<String, crate::ErrorKind> {
        if self.is_empty() {
            return Ok(self.generate_empty_report(&self.recording.format_with_credits(conn).await?));
        }

        Ok(self.generate_full_report(&self.recording.format_with_credits(conn).await?))
    }

    fn generate_empty_report(&self, name: &str) -> String {
        format!(
            "{}{}
            
The recording hasn't been listened to yet",
            "Statistics of ".black().on_green(),
            name.black().on_green(),
        )
    }

    fn generate_full_report(&self, name: &str) -> String {
        let conf = Config::load_or_panic();
        formatdoc! {"
            {pre_title}{statistics_of}{name_f}{post_title}
            
            [General]
               - Listen count: {listen_count}
               - First listened: {first_listened}
               - Last listened: {last_listened}
        
            [Listening rate]
               - Average duration between listens: {average_dur}
               - Estimated date of next listen: {next_listen_date}
               {overdue}

            [Radios]
               - Overdue score: {overdue_score}
               - Overdue score (with multiplier): {overdue_mul}
            ",
            pre_title = "".green().bold(),
            post_title = "".green().bold(),
            statistics_of = " Statistics of ".on_green().bold().black(),
            name_f = format!("{name} ").on_green().bold().black(),
            listen_count = self.listen_count(),
            first_listened = self.first_listen_date().unwrap().floor_to_second().with_timezone(&Local),
            last_listened = self.last_listen_date().unwrap().floor_to_second().with_timezone(&Local),
            average_dur = format_duration(
                self
                    .average_duration_between_listens()
                    .floor_to_minute()
                    .to_std()
                    .unwrap()
            ),
            next_listen_date =         self
            .estimated_date_of_next_listen()
            .unwrap()
            .floor_to_second(),
            overdue = get_overdue_line(self),
            overdue_score = self.overdue_factor().trunc_with_scale(2)  + Decimal::ONE,
            overdue_mul = ((self.overdue_factor() + Decimal::ONE)
            * conf.bumps.get_multiplier2(&RecordingMBID::from(
                self.recording().mbid.clone()
            )))
        .trunc_with_scale(2)
        }
    }
}

fn get_overdue_line(recording_info: &RecordingWithListens) -> String {
    let time = recording_info.overdue_by();

    if time <= Duration::zero() {
        return String::new();
    }

    format!(
        "    - Overdue by: {}",
        format_duration(time.floor_to_minute().to_std().unwrap())
    )
}
