use extend::ext;
use musicbrainz_rs::entity::{Browsable, BrowseResult};
use musicbrainz_rs::{BrowseQuery, Fetch};
use serde::de::DeserializeOwned;

#[ext]
pub impl<'a, T: Send> BrowseQuery<T> {
    #[allow(async_fn_in_trait)] // This is fine for this application, and if the lint is applied, it will complain about the huge async block...
    async fn execute_all(&mut self, limit: u8) -> color_eyre::Result<BrowseResult<T>>
    where
        T: Fetch<'a> + DeserializeOwned + Browsable + Clone,
    {
        self.limit(limit);
        let base_request = self.clone();

        let mut total_elements = None;
        let mut current_offset = 0_u16;
        let mut elements = Vec::new();

        while total_elements.is_none()
            || total_elements.is_some_and(|total| total > current_offset as i32)
        {
            let mut new_request = base_request.clone();
            new_request.offset(current_offset);

            let response = new_request.execute().await?;
            total_elements = Some(response.count);
            current_offset += limit as u16;
            elements.extend(response.entities);
        }

        Ok(BrowseResult {
            count: total_elements.unwrap_or(0),
            offset: 0,
            entities: elements,
        })
    }
}
