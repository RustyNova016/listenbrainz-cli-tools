use crate::core::entity_traits::config_file::ConfigFile;
use crate::error::ErrorKind;
use derive_getters::Getters;
use serde::Deserialize;
use serde::Serialize;

use super::Config;

#[derive(Debug, Serialize, Deserialize, Getters, Default)]
pub struct MapperConfig {
    /// List of recordings that shouldn't be proposed to be mapped
    pub(super) backlisted: Vec<String>,
}

impl Config {
    pub fn add_blacklisted_msid(msid: String) -> Result<(), ErrorKind> {
        let mut config = Self::load()?;

        let mut mapper_config = config.mapper.unwrap_or_default();
        mapper_config.backlisted.push(msid);

        config.mapper = Some(mapper_config);
        config.save()?;

        Ok(())
    }

    pub fn remove_blacklisted_msid(msid: &String) -> Result<(), ErrorKind> {
        let mut config = Self::load()?;

        let mut mapper_config = config.mapper.unwrap_or_default();
        mapper_config.backlisted.retain(|id| id != msid);

        config.mapper = Some(mapper_config);
        config.save()?;

        Ok(())
    }
}
