use crate::core::entity_traits::relations::HasRelationshipGeneric;
use crate::models::data::musicbrainz::artist::mbid::ArtistMBID;
use crate::models::data::musicbrainz::artist::Artist;
use crate::models::data::musicbrainz::relation::Relation;

impl Artist {
    pub async fn get_or_fetch_relations_misc(&self) -> color_eyre::Result<Vec<Relation>> {
        <Self as HasRelationshipGeneric<ArtistMBID, Vec<Relation>>>::get_or_fetch_relations(self)
            .await
    }
}

impl HasRelationshipGeneric<ArtistMBID, Vec<Relation>> for Artist {
    fn get_relations(&self) -> Option<Vec<Relation>> {
        self.relations.as_ref().cloned()
    }
}
