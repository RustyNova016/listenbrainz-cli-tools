use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::Arc;
use std::thread::panicking;

use crate::core::entity_traits::config_file::ConfigFile;
use once_cell::sync::Lazy;
use tokio::sync::RwLock;
use tokio::sync::RwLockReadGuard;
use tokio::sync::RwLockWriteGuard;

use super::Config;

pub(crate) static CONFIG: Lazy<GlobalConfig> = Lazy::new(GlobalConfig::load);

pub struct GlobalConfig {
    config: Arc<RwLock<Config>>,
}

impl GlobalConfig {
    pub fn load() -> Self {
        Self {
            config: Arc::new(RwLock::new(
                Config::load().expect("Couldn't load the configuration file"),
            )),
        }
    }

    pub async fn read(&self) -> RwLockReadGuard<Config> {
        self.config.read().await
    }

    pub async fn write(&self) -> GlobalConfigGuard<'_> {
        let guard = self.config.write().await;

        GlobalConfigGuard { config: guard }
    }
}

pub struct GlobalConfigGuard<'a> {
    pub(self) config: RwLockWriteGuard<'a, Config>,
}

impl<'a> Drop for GlobalConfigGuard<'a> {
    fn drop(&mut self) {
        if panicking() {
            return;
        }

        self.config.save().expect("Couldn't save config");
    }
}

impl<'a> Deref for GlobalConfigGuard<'a> {
    type Target = Config;

    fn deref(&self) -> &Self::Target {
        self.config.deref()
    }
}

impl<'a> DerefMut for GlobalConfigGuard<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.config.deref_mut()
    }
}
