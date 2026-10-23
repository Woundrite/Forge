use crate::error::ForgeError;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Config {
    #[serde(default)]
    pub plugin_dirs: Vec<PathBuf>,
}

pub fn config_path() -> Result<PathBuf, ForgeError> {
    ProjectDirs::from("", "", "Forge")
        .map(|dirs| dirs.config_dir().join("config.toml"))
        .ok_or_else(|| {
            ForgeError::Configuration("unable to determine the user configuration directory".into())
        })
}

pub fn load() -> Result<Config, ForgeError> {
    let path = config_path()?;
    if !path.exists() {
        return Ok(Config::default());
    }
    let contents = fs::read_to_string(path)?;
    Ok(toml::from_str(&contents)?)
}

pub fn save(config: &Config) -> Result<PathBuf, ForgeError> {
    let path = config_path()?;
    save_to(config, &path)?;
    Ok(path)
}

pub fn save_to(config: &Config, path: &Path) -> Result<(), ForgeError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let contents = toml::to_string_pretty(config)?;
    fs::write(path, contents)?;
    Ok(())
}
