pub mod external;
pub mod getters;
use itertools::Itertools;
use musicbrainz_rs::entity::alias::Alias;
use musicbrainz_rs::entity::artist::{ArtistType, Gender};
use musicbrainz_rs::entity::genre::Genre;
use musicbrainz_rs::entity::lifespan::LifeSpan;
use musicbrainz_rs::entity::tag::Tag;
use serde::{Deserialize, Serialize};

pub mod caching;
pub mod fetching;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Artist {
    pub id: String,
    pub name: String,
    pub sort_name: String,
    pub disambiguation: String,
    pub artist_type: Option<ArtistType>,
    pub gender: Option<Gender>,
    //pub area: Option<Area>,
    //pub begin_area: Option<Area>,
    //pub relations: Option<Vec<String>>,
    pub releases: Option<Vec<String>>,
    pub works: Option<Vec<String>>,
    pub release_groups: Option<Vec<String>>,
    recordings: Option<Vec<String>>,
    pub aliases: Option<Vec<Alias>>,
    pub tags: Option<Vec<Tag>>,
    pub genres: Option<Vec<Genre>>,
    //pub rating: Option<Rating>,
    pub country: Option<String>,
    pub annotation: Option<String>,
    pub life_span: Option<LifeSpan>,
}

impl From<musicbrainz_rs::entity::artist::Artist> for Artist {
    fn from(artist: musicbrainz_rs::entity::artist::Artist) -> Self {
        Self {
            id: artist.id,
            name: artist.name,
            aliases: artist.aliases,
            annotation: artist.annotation,
            artist_type: artist.artist_type,
            country: artist.country,
            disambiguation: artist.disambiguation,
            gender: artist.gender,
            genres: artist.genres,
            life_span: artist.life_span,
            recordings: artist.recordings.map(|recodings| {
                recodings
                    .into_iter()
                    .map(|recording| recording.id)
                    .collect_vec()
            }),
            release_groups: artist.release_groups.map(|release_groups| {
                release_groups
                    .into_iter()
                    .map(|release_group| release_group.id)
                    .collect_vec()
            }),
            releases: artist
                .releases
                .map(|releases| releases.into_iter().map(|release| release.id).collect_vec()),
            sort_name: artist.sort_name,
            tags: artist.tags,
            works: artist
                .works
                .map(|works| works.into_iter().map(|work| work.id).collect_vec()),
            //relations: artist.relations.map(|relations| relations.into_iter().map(|relation| relation.).collect_vec()),
        }
    }
}
