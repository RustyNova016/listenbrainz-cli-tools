use crate::models::data::musicbrainz::musicbrainz_entity::MusicBrainzEntity;

pub trait FlattenExternal {
    fn flattened(&self) -> (MusicBrainzEntity, Vec<MusicBrainzEntity>);
}
