use crate::models::data::musicbrainz::recording::mbid::RecordingMBID;

use super::ListenCollection;

impl ListenCollection {
    pub async fn get_listened_recordings_mbids(&self) -> color_eyre::Result<Vec<RecordingMBID>> {
        //TODO: Multithread it
        let mut recordings = Vec::new();

        for listen in self.get_mapped_listens().iter() {
            recordings.push(
                listen
                    .get_primary_recording_id()
                    .await?
                    .expect("Listen should be mapped"),
            );
        }

        Ok(recordings)
    }
}
