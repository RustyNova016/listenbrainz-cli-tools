use derive_more::{Deref, DerefMut, From, Into};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Deref, DerefMut, Into, From, Serialize, Deserialize)]
pub struct ArtistMBID(String);
