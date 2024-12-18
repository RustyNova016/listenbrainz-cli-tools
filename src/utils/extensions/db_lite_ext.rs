use extend::ext;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;
use musicbrainz_db_lite::models::musicbrainz::relations::Relation;

#[ext]
pub impl Relation<Recording, Recording> {
    fn is_remix_of_rel(&self, base_recording: &Recording) -> bool {
        // Sanity check: The relation must be a remix relation, and the entity0 is the recording we are querying
        self.relation_type == "remix"
            && self.entity0 == base_recording.id
            && self.direction == "forward"
    }
}
