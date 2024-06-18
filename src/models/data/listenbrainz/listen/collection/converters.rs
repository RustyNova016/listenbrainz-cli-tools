use itertools::Itertools;

use crate::models::data::listenbrainz::listen::listen_unspe::ListenMappingState;
use crate::models::data::musicbrainz::recording::mbid::RecordingMBID;

use super::ListenCollection;
use super::MappedListensCollection::MappedNaiveListensCollection;

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

    pub fn into_listen_mapping_state_vec(self) -> Vec<ListenMappingState> {
        self.into()
    }
}

impl From<ListenCollection> for Vec<ListenMappingState> {
    fn from(value: ListenCollection) -> Self {
        value
            .into_iter()
            .map(|listen| listen.as_ref().clone().into())
            .collect_vec()
    }
}
