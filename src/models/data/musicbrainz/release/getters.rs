use super::Release;

impl Release {
    pub fn get_mbid(&self) -> &str {
        &self.id
    }
}
