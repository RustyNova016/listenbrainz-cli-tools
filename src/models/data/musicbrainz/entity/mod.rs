use std::marker::PhantomData;

use serde::de::DeserializeOwned;
use serde::Serialize;

use super::mbid::IsMusicbrainzID;

pub struct MusicBrainEntity<K: IsMusicbrainzID<T>, T: IsMusicbrainzEntity<K>> {
    data: T,
    key: PhantomData<K>,
}

pub trait IsMusicbrainzEntity<K>
where
    Self: Serialize + DeserializeOwned + Clone,
    K: IsMusicbrainzID<Self>,
{
    fn get_mbid(&self) -> &K;
}
