use extend::ext;
use musicbrainz_rs::{
    entity::{Browsable, BrowseResult},
    BrowseQuery, Fetch,
};
use serde::de::DeserializeOwned;

#[ext]
pub impl<'a, T> BrowseQuery<T> {
    async fn execute_all(&mut self, limit: u8) -> color_eyre::Result<BrowseResult<T>>
    where
        T: Fetch<'a> + DeserializeOwned + Browsable + Clone,
    {
        self.limit(limit);
        let mut total_elements = None;
        let mut current_offset = 0_u32;
        let mut elements = Vec::new();

        while total_elements.is_none()
            || total_elements.is_some_and(|total| total > current_offset as i32)
        {
            let response = self.execute().await?;
            total_elements = Some(response.count);
            current_offset += limit as u32;
            elements.extend(response.entities)
        }

        Ok(BrowseResult {
            count: total_elements.unwrap_or(0),
            offset: 0,
            entities: elements,
        })
    }
}
