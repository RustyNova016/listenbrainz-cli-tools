use super::listens::collection::UserListenCollection;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DataStore {
    recordings: UserListenCollection,
}
