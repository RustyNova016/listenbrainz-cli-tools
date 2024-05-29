use super::Relation;
use extend::ext;

#[ext]
pub impl Vec<Relation> {
    fn find_relation_type_id(&self, id: &str) -> Option<&Relation> {
        self.iter().find(|rel| rel.type_id().as_str() == id)
    }
}
