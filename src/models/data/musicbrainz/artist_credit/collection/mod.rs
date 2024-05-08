use std::ops::Deref;
use std::sync::Arc;

use itertools::Itertools;
use serde::{Deserialize, Serialize};
use crate::models::data::musicbrainz::artist::mbid::ArtistMBID;

use super::ArtistCredit;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct ArtistCredits(Vec<Arc<ArtistCredit>>);

impl Deref for ArtistCredits {
    type Target = Vec<Arc<ArtistCredit>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ArtistCredits {
    pub fn get_artist_ids(&self) -> Vec<ArtistMBID> {
        self.iter()
            .map(|credit| credit.artist.clone())
            .collect_vec()
    }
}

impl<T: Into<ArtistCredit>> From<Vec<T>> for ArtistCredits {
    fn from(value: Vec<T>) -> Self {
        Self(
            value
                .into_iter()
                .map(|element| Arc::new(element.into()))
                .collect_vec(),
        )
    }
}
