use extend::ext;
use itertools::Itertools;
use crate::models::data::musicbrainz::relation::Relation;

pub trait HasRelationships {
    fn get_relationships(&self) -> Vec<Relation>;

    fn get_url_relations(&self) -> Vec<Relation> {
        self.get_relationships().into_iter().filter(|relation| relation.content.is_url()).collect_vec()
    }
}

#[ext]
pub impl<T: HasRelationships> Vec<T> {
    fn into_all_relationships(self) -> Vec<Relation> {
        self.into_iter().flat_map(|item| item.get_relationships()).cloned().collect_vec()
    }
}