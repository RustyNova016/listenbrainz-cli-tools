use chrono::DateTime;
use chrono::Duration;
use chrono::OutOfRangeError;
use chrono::Utc;
use extend::ext;

#[ext]
pub impl Duration {
    fn from_human_string(value: &str) -> color_eyre::Result<Duration> {
        let human_dur: humantime::Duration = value.parse()?;
        Ok(Duration::from_std(*human_dur)?)
    }

    fn to_humantime(self) -> Result<humantime::Duration, OutOfRangeError> {
        Ok(humantime::Duration::from(self.to_std()?))
    }

    #[must_use]
    fn floor_to_minute(self) -> Self {
        Self::minutes(self.num_minutes())
    }
}

#[ext]
pub impl DateTime<Utc> {
    #[must_use]
    fn floor_to_second(self) -> Self {
        Self::from_timestamp(self.timestamp(), 0).unwrap()
    }
}
