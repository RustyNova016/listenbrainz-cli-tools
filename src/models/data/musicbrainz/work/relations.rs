use crate::core::entity_traits::relations::HasRelationshipGeneric;
use crate::models::data::musicbrainz::relation::Relation;
use crate::models::data::musicbrainz::work::mbid::WorkMBID;
use crate::models::data::musicbrainz::work::Work;

impl Work {
    pub async fn get_or_fetch_relations_misc(&self) -> color_eyre::Result<Vec<Relation>> {
        <Self as HasRelationshipGeneric<WorkMBID, Vec<Relation>>>::get_or_fetch_relations(self)
            .await
    }
}

impl HasRelationshipGeneric<WorkMBID, Vec<Relation>> for Work {
    fn get_relations(&self) -> Option<Vec<Relation>> {
        self.relations.as_ref().cloned()
    }
}
