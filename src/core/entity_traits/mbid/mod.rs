use crate::models::data::musicbrainz::external_musicbrainz_entity::ExternalMusicBrainzEntity;
use crate::models::data::musicbrainz::mbid::MBIDEnum;
use crate::models::data::musicbrainz::musicbrainz_entity::MusicbrainzEntityKind;
use extend::ext;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::{Debug, Display};
use std::future::Future;
use std::hash::Hash;
use std::sync::Arc;

use crate::models::data::musicbrainz::musicbrainz_entity::AnyMusicBrainzEntity;

use super::updatable::Updatable;

pub mod is_cached_mbid;

pub trait IsMbid<T>
where
    Self: Display + Clone + Serialize + DeserializeOwned + Eq + Hash,
    T: HasMBID<Self>,
{
    fn get_or_fetch_entity(&self) -> impl Future<Output = color_eyre::Result<Arc<T>>> + Send;

    fn fetch(&self) -> impl Future<Output = color_eyre::Result<ExternalMusicBrainzEntity>> + Send;

    fn into_mbid(self) -> MBIDEnum;

    fn get_kind(&self) -> MusicbrainzEntityKind;
}

#[ext]
pub impl<T, I> Vec<I>
where
    T: HasMBID<I>,
    I: IsMbid<T>,
{
    #[allow(async_fn_in_trait)]
    async fn get_or_fetch_entities(&self) -> color_eyre::Result<Vec<Arc<T>>> {
        let mut result = Vec::new();

        for item in self {
            result.push(item.get_or_fetch_entity().await?);
        }

        Ok(result)
    }
}

pub trait HasMBID<K>
where
    Self:Serialize + DeserializeOwned + Updatable + Sized + Debug + Clone + Into<AnyMusicBrainzEntity>,
    Arc<Self>: Into<AnyMusicBrainzEntity> + TryFrom<AnyMusicBrainzEntity>,
    K: IsMbid<Self>,
{
    fn get_mbid(&self) -> K;

    fn into_generic(self) -> AnyMusicBrainzEntity {
        self.into()
    }

    fn get_kind(&self) -> MusicbrainzEntityKind {
        self.get_mbid().get_kind()
    }
}
