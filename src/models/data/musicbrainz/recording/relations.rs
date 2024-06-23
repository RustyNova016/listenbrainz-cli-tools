use crate::core::entity_traits::relations::HasRelationshipGeneric;
use crate::models::data::musicbrainz::artist_credit::collection::ArtistCredits;
use crate::models::data::musicbrainz::recording::mbid::RecordingMBID;
use crate::models::data::musicbrainz::recording::Recording;
use crate::models::data::musicbrainz::relation::Relation;
use crate::models::data::musicbrainz::release::mbid::ReleaseMBID;

impl Recording {
    pub async fn get_or_fetch_artist_credits(&self) -> color_eyre::Result<ArtistCredits> {
        <Self as HasRelationshipGeneric<RecordingMBID, ArtistCredits>>::get_or_fetch_relations(self)
            .await
    }

    pub async fn get_or_fetch_releases_mbids(&self) -> color_eyre::Result<Vec<ReleaseMBID>> {
        <Self as HasRelationshipGeneric<RecordingMBID, Vec<ReleaseMBID>>>::get_or_fetch_relations(
            self,
        )
        .await
    }

    pub async fn get_or_fetch_relations_misc(&self) -> color_eyre::Result<Vec<Relation>> {
        <Self as HasRelationshipGeneric<RecordingMBID, Vec<Relation>>>::get_or_fetch_relations(self)
            .await
    }
}

impl HasRelationshipGeneric<RecordingMBID, ArtistCredits> for Recording {
    fn get_relations(&self) -> Option<ArtistCredits> {
        self.artist_credit.as_ref().cloned()
    }
}

impl HasRelationshipGeneric<RecordingMBID, Vec<ReleaseMBID>> for Recording {
    fn get_relations(&self) -> Option<Vec<ReleaseMBID>> {
        self.releases.as_ref().cloned()
    }
}

impl HasRelationshipGeneric<RecordingMBID, Vec<Relation>> for Recording {
    fn get_relations(&self) -> Option<Vec<Relation>> {
        self.relations.as_ref().cloned()
    }
}
