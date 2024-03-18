use listenbrainz::raw::response::UserListensListen;

use super::UserListen;

/// Collection of listens
pub struct UserListenCollection {
    data: Vec<UserListen>,
}

impl UserListenCollection {
    pub fn get_mapped_listens(&self) -> impl Iterator<Item = &'_ UserListen> {
        self.data.iter().filter(|element| element.is_mapped())
    }
}

impl TryFrom<Vec<UserListensListen>> for UserListenCollection {
    type Error = &'static str;

    fn try_from(value: Vec<UserListensListen>) -> Result<Self, Self::Error> {
        let mut data: Vec<UserListen> = Vec::new();

        for listen in value.into_iter() {
            data.push(listen.try_into()?)
        }

        Ok(Self { data })
    }
}
