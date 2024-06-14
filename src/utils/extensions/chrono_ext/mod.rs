use chrono::Duration;
use extend::ext;

#[ext]
pub impl Duration {
    fn from_human_string(value: &str) -> color_eyre::Result<Duration> {
        let human_dur: humantime::Duration = value.parse()?;
        Ok(Duration::from_std(*human_dur)?)
    }
}
