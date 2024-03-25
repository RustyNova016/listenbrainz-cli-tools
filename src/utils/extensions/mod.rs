use chrono::{DateTime, TimeZone, Utc};
use extend::ext;
use listenbrainz::raw::response::{UserListensListen, UserListensPayload};

#[ext]
pub impl UserListensPayload {
    /// Return the oldest
    fn get_date_of_oldest_listen_of_user(&self) -> DateTime<Utc> {
        Utc.timestamp_opt(self.oldest_listen_ts, 0)
            .single()
            .expect("Error: Could not parse listen's timestamp")
    }

    fn get_date_of_latest_listen_of_user(&self) -> DateTime<Utc> {
        Utc.timestamp_opt(self.latest_listen_ts, 0)
            .single()
            .expect("Error: Could not parse listen's timestamp")
    }

    fn get_oldest_payload_listen(&self) -> Option<&UserListensListen> {
        self.listens.iter().min_by_key(|listen| listen.listened_at)
    }

    fn get_latest_payload_listen(&self) -> Option<&UserListensListen> {
        self.listens.iter().max_by_key(|listen| listen.listened_at)
    }

    fn get_date_of_oldest_listen_of_payload(&self) -> Option<DateTime<Utc>> {
        self.get_oldest_payload_listen()
            .and_then(|listen| Utc.timestamp_opt(listen.listened_at, 0).single())
    }

    fn get_date_of_latest_listen_of_payload(&self) -> Option<DateTime<Utc>> {
        self.get_latest_payload_listen()
            .and_then(|listen| Utc.timestamp_opt(listen.listened_at, 0).single())
    }
}

#[ext]
pub impl UserListensListen {
    fn get_listened_at_date(&self) -> DateTime<Utc> {
        Utc.timestamp_opt(self.listened_at, 0)
            .single()
            .expect("Error: Could not parse listen's timestamp")
    }
}
