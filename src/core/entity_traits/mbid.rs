use extend::ext;

pub trait MBID<T> {
    fn get_or_fetch_entity(
        &self,
    ) -> impl std::future::Future<Output = color_eyre::Result<T>> + Send;
}

#[ext]
pub impl<T, I: MBID<T>> Vec<I> {
    #[allow(async_fn_in_trait)]
    async fn get_or_fetch_entities(&self) -> color_eyre::Result<Vec<T>> {
        let mut result = Vec::new();

        for item in self {
            result.push(item.get_or_fetch_entity().await?);
        }

        Ok(result)
    }
}
