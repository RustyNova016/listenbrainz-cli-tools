pub mod external;

use derive_getters::Getters;
use serde::{Deserialize, Serialize};

use self::media::Media;

pub mod caching;
pub mod converters;
pub mod fetching;
pub mod getters;
pub mod media;
pub mod track;
pub mod mbid;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize, Getters)]
pub struct Release {
    id: String,
    title: String,
    status_id: Option<String>,
    //status: Option<ReleaseStatus>,
    //date: Option<NaiveDate>,
    country: Option<String>,
    //quality: Option<ReleaseQuality>,
    barcode: Option<String>,
    disambiguation: Option<String>,
    packaging_id: Option<String>,
    //packaging: Option<ReleasePackaging>,
    //relations: Option<Vec<Relation>>,
    //release_group: Option<ReleaseGroup>,
    //artist_credit: Option<Vec<ArtistCredit>>,
    media: Option<Vec<Media>>,
    //label_info: Option<Vec<LabelInfo>>,
    //tags: Option<Vec<Tag>>,
    //aliases: Option<Vec<Alias>>,
    //genres: Option<Vec<Genre>>,
    annotation: Option<String>,
}
