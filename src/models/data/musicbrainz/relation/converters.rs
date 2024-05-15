use musicbrainz_rs::entity::relations::Relation as RelationMS;
use musicbrainz_rs::entity::relations::RelationContent;

use super::Relation;
use super::RelationTarget;

impl From<RelationMS> for Relation {
    fn from(value: RelationMS) -> Self {
        Self {
            attribute_ids: value.attribute_ids,
            attribute_values: value.attribute_values,
            attributes: value.attributes,
            begin: value.begin,
            content: value.content.into(),
            direction: value.direction,
            end: value.end,
            ended: value.ended,
            relation_type: value.relation_type,
            source_credit: value.source_credit,
            target_credit: value.target_credit,
            target_type: value.target_type,
            type_id: value.type_id,
        }
    }
}

impl From<RelationContent> for RelationTarget {
    fn from(value: RelationContent) -> Self {
        match value {
            RelationContent::Area(_) => Self::Area(),
            RelationContent::Artist(val) => Self::Artist(val.id.into()),
            RelationContent::Event(_) => Self::Event(),
            RelationContent::Label(_) => Self::Label(),
            RelationContent::Place(_) => Self::Place(),
            RelationContent::Recording(val) => Self::Recording(val.id.into()),
            RelationContent::Release(val) => Self::Release(val.id.into()),
            RelationContent::ReleaseGroup(val) => Self::ReleaseGroup(val.id.into()),
            RelationContent::Series(_) => Self::Series(),
            RelationContent::Url(_) => Self::Url(),
            RelationContent::Work(val) => Self::Work(val.id.into()),
        }
    }
}
