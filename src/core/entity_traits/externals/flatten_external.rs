use crate::models::data::musicbrainz::musicbrainz_entity::AnyMusicBrainzEntity;

pub trait FlattenExternal {
    fn flattened(&self) -> (AnyMusicBrainzEntity, Vec<AnyMusicBrainzEntity>);
}
