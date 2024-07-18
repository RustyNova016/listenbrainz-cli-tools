use std::sync::Arc;

use chrono::DateTime;
use chrono::Utc;
use extend::ext;

use crate::models::data::listenbrainz::listen::Listen;

#[ext(name=ListenIterator)]
pub impl<T> T
where
    T: Iterator<Item = Arc<Listen>>,
{
    /// Return all the listens newer or equal to the specified time in a new self
    fn where_newer_or_equal_than(self, time: &DateTime<Utc>) -> impl Iterator<Item = Arc<Listen>> {
        self.filter(|listen| &listen.listened_at >= time)
    }

    /// Return all the listens older or equal to the specified time in a new self
    fn where_older_or_equal_than(self, time: &DateTime<Utc>) -> impl Iterator<Item = Arc<Listen>> {
        self.filter(|listen| &listen.listened_at <= time)
    }
}
