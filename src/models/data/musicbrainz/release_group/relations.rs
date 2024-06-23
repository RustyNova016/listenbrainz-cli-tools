use crate::core::entity_traits::relations::HasRelationshipGeneric;
use crate::models::data::musicbrainz::artist_credit::collection::ArtistCredits;
use crate::models::data::musicbrainz::relation::Relation;
use crate::models::data::musicbrainz::release::mbid::ReleaseMBID;
use crate::models::data::musicbrainz::release_group::mbid::ReleaseGroupMBID;
use crate::models::data::musicbrainz::release_group::ReleaseGroup;

impl ReleaseGroup {
    pub async fn get_or_fetch_artist_credits(&self) -> color_eyre::Result<ArtistCredits> {
        <Self as HasRelationshipGeneric<ReleaseGroupMBID, ArtistCredits>>::get_or_fetch_relations(
            self,
        )
        .await
    }

    pub async fn get_or_fetch_releases_mbids(&self) -> color_eyre::Result<Vec<ReleaseMBID>> {
        <Self as HasRelationshipGeneric<ReleaseGroupMBID, Vec<ReleaseMBID>>>::get_or_fetch_relations(self).await
    }

    pub async fn get_or_fetch_relations_misc(&self) -> color_eyre::Result<Vec<Relation>> {
        <Self as HasRelationshipGeneric<ReleaseGroupMBID, Vec<Relation>>>::get_or_fetch_relations(
            self,
        )
        .await
    }
}

impl HasRelationshipGeneric<ReleaseGroupMBID, ArtistCredits> for ReleaseGroup {
    fn get_relations(&self) -> Option<ArtistCredits> {
        self.artist_credit.as_ref().cloned()
    }
}

impl HasRelationshipGeneric<ReleaseGroupMBID, Vec<ReleaseMBID>> for ReleaseGroup {
    fn get_relations(&self) -> Option<Vec<ReleaseMBID>> {
        self.releases.as_ref().cloned()
    }
}

impl HasRelationshipGeneric<ReleaseGroupMBID, Vec<Relation>> for ReleaseGroup {
    fn get_relations(&self) -> Option<Vec<Relation>> {
        self.relations.as_ref().cloned()
    }
}
