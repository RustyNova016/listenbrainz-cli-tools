use derive_more::{Deref, DerefMut, Display, From, Into};
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, PartialEq, Eq, Deref, DerefMut, Into, From, Serialize, Deserialize, Display,
)]
pub struct ArtistMBID(String);
