use chrono::prelude::DateTime;
use chrono::prelude::Utc;

use crate::models::data::musicbrainz::recording::mbid::RecordingMBID;

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

    /// Filter listens depending on their mapped recordings. Recording ids are assumed to be primary
    #[deprecated]
    pub async fn filter_recordings(
        &self,
        list: &[RecordingMBID],
        is_blacklist: bool,
        keep_unmapped: bool,
    ) -> color_eyre::Result<Self> {
        //TODO: Multithread it
        let mut result = Self::default();

        for listen in self.iter() {
            let Some(listen_id) = listen.get_primary_recording_id().await? else {
                if keep_unmapped {
                    result.push(listen.clone());
                }

                continue;
            };

            let contains = list.contains(&listen_id);

            if (!contains && is_blacklist) || (contains && !is_blacklist) {
                result.push(listen.clone());
            }
        }

        Ok(result)
    }
}
