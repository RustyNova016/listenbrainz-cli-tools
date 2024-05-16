use chrono::prelude::DateTime;
use chrono::prelude::Utc;

use super::ListenCollection;

impl ListenCollection {
    /// Only retain mapped listens
    pub fn retain_mapped(&mut self) -> &mut Self {
        self.retain(|listen| listen.is_mapped());
        self
    }

    pub fn get_listened_after(&self, datetime: &DateTime<Utc>) -> Self {
        self.iter()
            .filter(|listen| listen.get_listened_at() > datetime)
            .cloned()
            .collect()
    }
}
