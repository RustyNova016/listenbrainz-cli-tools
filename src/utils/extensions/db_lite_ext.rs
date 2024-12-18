use extend::ext;
use musicbrainz_db_lite::models::musicbrainz::artist::Artist;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;
use musicbrainz_db_lite::models::musicbrainz::relations::Relation;

#[ext]
pub impl Relation<Recording, Recording> {
    fn is_remix_of_rel(&self, base_recording: &Recording) -> bool {
        self.relation_type == "remix"
            && self.entity0 == base_recording.id
            && self.direction == "forward"
    }
}

#[ext]
pub impl Relation<Recording, Artist> {
    fn is_remixer_rel(&self, base_recording: &Recording) -> bool {
        self.relation_type == "remixer"
            && self.entity1 == base_recording.id
            && self.direction == "backward"
    }
}
