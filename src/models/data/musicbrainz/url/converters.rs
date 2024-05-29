use musicbrainz_rs::entity::url::Url;

impl From<Url> for super::URL {
    fn from(value: Url) -> Self {
        Self {
            id: value.id,
            tags: value.tags,
            resource: value.resource,
        }
    }
}
