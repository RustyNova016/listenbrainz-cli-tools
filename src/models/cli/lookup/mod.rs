use crate::models::data::musicbrainz::entity::entity_kind::MusicbrainzEntityKind;
use crate::models::data::musicbrainz::mbid::MBID;
use crate::tools::lookup::lookup_command;
use clap::Parser;
use clap::ValueEnum;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct LookupCommand {
    /// Name of the user to look up stats from
    username: String,

    /// The type of entity to look for
    entity_type: LookupTarget,

    /// The id of the entity (Accept URLs)
    id: String,
}

impl LookupCommand {
    pub async fn run(&self) -> color_eyre::Result<()> {
        let id = match self.entity_type {
            LookupTarget::Recording => {
                MBID::from_string(&self.id, MusicbrainzEntityKind::Recording)?
            }
        };

        lookup_command(&self.username, id).await?;
        Ok(())
    }
}

#[derive(ValueEnum, Clone, Debug, Copy)]
pub enum LookupTarget {
    Recording,
}
