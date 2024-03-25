use chrono::{DateTime, TimeZone, Utc};
use extend::ext;
use listenbrainz::raw::response::UserListensPayload;

#[ext]
pub impl UserListensPayload {
    fn get_oldest_listen_date(&self) -> DateTime<Utc> {
        Utc.timestamp_opt(self.oldest_listen_ts, 0)
            .single()
            .expect("Error: Could not parse listen's timestamp")
    }

    fn get_latest_listen_date(&self) -> DateTime<Utc> {
        Utc.timestamp_opt(self.latest_listen_ts, 0)
            .single()
            .expect("Error: Could not parse listen's timestamp")
    }
}
