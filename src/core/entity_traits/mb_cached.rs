use std::sync::Arc;

use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::core::entity_traits::mbid::{HasMBID, IsMbid};
use crate::models::data::musicbrainz::entity::is_musicbrainz_entity::IsMusicbrainzEntity;

pub trait MBCached<K>
where
    K: IsMbid<Self> + Serialize + DeserializeOwned,
    Self: Serialize + DeserializeOwned + HasMBID<K> + IsMusicbrainzEntity + Clone,
{




    


}
