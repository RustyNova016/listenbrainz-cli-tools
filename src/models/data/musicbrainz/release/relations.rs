use crate::core::entity_traits::relations::HasRelationshipGeneric;
use crate::models::data::musicbrainz::artist_credit::collection::ArtistCredits;
use crate::models::data::musicbrainz::relation::Relation;
use crate::models::data::musicbrainz::release::mbid::ReleaseMBID;
use crate::models::data::musicbrainz::release::Release;
use crate::models::data::musicbrainz::release_group::mbid::ReleaseGroupMBID;

impl Release {
    pub async fn get_or_fetch_artist_credits(&self) -> color_eyre::Result<ArtistCredits> {
        <Self as HasRelationshipGeneric<ReleaseMBID, ArtistCredits>>::get_or_fetch_relations(self)
            .await
    }

    pub async fn get_or_fetch_release_group_mbid(&self) -> color_eyre::Result<ReleaseGroupMBID> {
        <Self as HasRelationshipGeneric<ReleaseMBID, ReleaseGroupMBID>>::get_or_fetch_relations(
            self,
        )
        .await
    }

    pub async fn get_or_fetch_relations_misc(&self) -> color_eyre::Result<Vec<Relation>> {
        <Self as HasRelationshipGeneric<ReleaseMBID, Vec<Relation>>>::get_or_fetch_relations(self)
            .await
    }
}

impl HasRelationshipGeneric<ReleaseMBID, ArtistCredits> for Release {
    fn get_relations(&self) -> Option<ArtistCredits> {
        self.artist_credit.as_ref().cloned()
    }
}

impl HasRelationshipGeneric<ReleaseMBID, ReleaseGroupMBID> for Release {
    fn get_relations(&self) -> Option<ReleaseGroupMBID> {
        self.release_group.as_ref().cloned()
    }
}

impl HasRelationshipGeneric<ReleaseMBID, Vec<Relation>> for Release {
    fn get_relations(&self) -> Option<Vec<Relation>> {
        self.relations.as_ref().cloned()
    }
}
