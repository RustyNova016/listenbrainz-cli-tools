use std::sync::Arc;

use chrono::DateTime;
use chrono::Utc;
use extend::ext;
use itertools::Itertools;

use crate::models::data::listenbrainz::listen::listen_spe::ListenSpe;
use crate::models::data::listenbrainz::listen::listen_spe::MappingState;

#[ext(name = ListenCollectionCommons)]
pub impl<S: MappingState> Vec<Arc<ListenSpe<S>>> {
    fn extract_ref_listened_after(self, date: &DateTime<Utc>) -> Self {
        self.into_iter()
            .filter(|listen| listen.listened_at() > date)
            .collect_vec()
    }

    fn oldest_listen(&self) -> Option<&Arc<ListenSpe<S>>> {
        self.iter().min_by_key(|listen| listen.listened_at())
    }

    fn latest_listen(&self) -> Option<&Arc<ListenSpe<S>>> {
        self.iter().max_by_key(|listen| listen.listened_at())
    }
}
