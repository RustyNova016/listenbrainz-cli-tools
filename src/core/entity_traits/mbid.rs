use crate::models::data::musicbrainz::external_musicbrainz_entity::ExternalMusicBrainzEntity;
use crate::models::data::musicbrainz::mbid::MBID;
use extend::ext;
use std::fmt::Display;
use std::future::Future;

pub trait IsMbid<T>: Display + Clone {
    fn get_or_fetch_entity(&self) -> impl Future<Output = color_eyre::Result<T>> + Send;

    fn fetch(&self) -> impl Future<Output = color_eyre::Result<ExternalMusicBrainzEntity>> + Send;

    fn into_mbid(self) -> MBID;
}

#[ext]
pub impl<T, I: IsMbid<T>> Vec<I> {
    #[allow(async_fn_in_trait)]
    async fn get_or_fetch_entities(&self) -> color_eyre::Result<Vec<T>> {
        let mut result = Vec::new();

        for item in self {
            result.push(item.get_or_fetch_entity().await?);
        }

        Ok(result)
    }
}

pub trait HasMBID<K: IsMbid<Self>>: Sized {
    fn get_mbid(&self) -> K;
}
