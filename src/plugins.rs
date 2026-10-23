use crate::error::ForgeError;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Component, Path, PathBuf},
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PluginManifest {
    pub plugin: PluginMetadata,
    #[serde(default)]
    pub commands: std::collections::BTreeMap<String, PluginCommand>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PluginMetadata {
    pub name: String,
    pub version: String,
    pub description: String,
    pub api_version: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PluginCommand {
    pub executable: PathBuf,
    pub description: String,
}

#[derive(Clone, Debug)]
pub struct DiscoveredCommand {
    pub name: String,
    pub description: String,
    pub executable: PathBuf,
    pub plugin_name: String,
}

pub fn discover(directories: &[PathBuf]) -> Result<Vec<DiscoveredCommand>, ForgeError> {
    let mut commands = Vec::new();
    for directory in directories {
        if !directory.exists() {
            continue;
        }
        let entries = fs::read_dir(directory).map_err(|error| {
            ForgeError::Plugin(format!(
                "cannot read plugin directory {}: {error}",
                directory.display()
            ))
        })?;
        for entry in entries {
            let entry = entry.map_err(|error| ForgeError::Plugin(error.to_string()))?;
            if !entry.file_type()?.is_dir() {
                continue;
            }
            let plugin_dir = entry.path();
            let manifest_path = plugin_dir.join("plugin.toml");
            if !manifest_path.is_file() {
                continue;
            }
            let manifest: PluginManifest = toml::from_str(&fs::read_to_string(&manifest_path)?)
                .map_err(|error| {
                    ForgeError::Plugin(format!("{}: {error}", manifest_path.display()))
                })?;
            validate_manifest(&manifest, &plugin_dir)?;
            for (name, command) in manifest.commands {
                let executable = validate_executable_path(&plugin_dir, &command.executable)?;
                commands.push(DiscoveredCommand {
                    name,
                    description: command.description,
                    executable,
                    plugin_name: manifest.plugin.name.clone(),
                });
            }
        }
    }
    commands.sort_by(|left, right| left.name.cmp(&right.name));
    for pair in commands.windows(2) {
        if pair[0].name == pair[1].name {
            return Err(ForgeError::Plugin(format!(
                "duplicate plugin command '{}'",
                pair[0].name
            )));
        }
    }
    Ok(commands)
}

fn validate_manifest(manifest: &PluginManifest, directory: &Path) -> Result<(), ForgeError> {
    if manifest.plugin.name.trim().is_empty()
        || manifest.plugin.version.trim().is_empty()
        || manifest.plugin.api_version != 1
    {
        return Err(ForgeError::Plugin(format!(
            "invalid manifest in {}",
            directory.display()
        )));
    }
    if manifest.commands.is_empty() {
        return Err(ForgeError::Plugin(format!(
            "plugin '{}' registers no commands",
            manifest.plugin.name
        )));
    }
    for name in manifest.commands.keys() {
        if name.trim().is_empty() || name.contains([' ', '\0']) {
            return Err(ForgeError::Plugin(format!("invalid command name '{name}'")));
        }
    }
    Ok(())
}

fn validate_executable_path(plugin_dir: &Path, configured: &Path) -> Result<PathBuf, ForgeError> {
    if configured.is_absolute()
        || configured
            .components()
            .any(|component| matches!(component, Component::ParentDir))
    {
        return Err(ForgeError::Plugin(format!(
            "plugin executable '{}' must stay inside its plugin directory",
            configured.display()
        )));
    }
    let path = plugin_dir.join(configured);
    if !path.is_file() {
        return Err(ForgeError::Plugin(format!(
            "plugin executable does not exist: {}",
            path.display()
        )));
    }
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn rejects_executable_path_traversal() {
        let root = tempdir().unwrap();
        let plugin = root.path().join("sample");
        fs::create_dir(&plugin).unwrap();
        fs::write(
            plugin.join("plugin.toml"),
            "[plugin]\nname='sample'\nversion='1'\ndescription='sample'\napi_version=1\n[commands.run]\nexecutable='../run.exe'\ndescription='run'\n",
        )
        .unwrap();
        let error = discover(&[root.path().to_path_buf()]).unwrap_err();
        assert!(error.to_string().contains("must stay inside"));
    }
}
