pub mod entity_cache;
pub mod musicbrainz_cache;
pub mod serde_cacache;

use directories::BaseDirs;
use once_cell::sync::Lazy;
use std::fs;
use std::path::PathBuf;

pub static CACHE_LOCATION: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = BaseDirs::new()
        .expect("Couldn't find standard directory. Is your system an oddball one?")
        .cache_dir()
        .to_path_buf();
    path.push("listenbrainz_cli_tools");
    fs::create_dir_all(&path).expect("Couldn't create cache directory");
    path
});

pub static CONFIG_FILE: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = BaseDirs::new()
        .expect("Couldn't find standard directory. Is your system an oddball one?")
        .config_dir()
        .to_path_buf();
    path.push("listenbrainz_cli_tools");
    fs::create_dir_all(&path).expect("Couldn't create config directory");
    path.push("config.json");
    path
});
