use crate::{
    config::{self, Config},
    error::ForgeError,
    output::{self, OutputFormat},
    plugins::{self, DiscoveredCommand},
    protocol,
};
use serde::Serialize;
use std::{
    collections::BTreeMap,
    env,
    path::PathBuf,
    process::{Command, Stdio},
};

const ENGINE_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Clone, Copy, Debug)]
pub struct EngineOptions {
    pub output: OutputFormat,
    pub detached: bool,
    pub newshell: bool,
    pub wait: bool,
    pub capture: bool,
}

pub struct Engine {
    config: Config,
    commands: BTreeMap<String, DiscoveredCommand>,
}

#[derive(Serialize)]
struct VersionInfo<'a> {
    engine_version: &'a str,
    command_api_version: u32,
    shell_protocol_version: u32,
}

#[derive(Serialize)]
struct PluginInfo<'a> {
    command: &'a str,
    plugin: &'a str,
    description: &'a str,
}

impl Engine {
    pub fn load() -> Result<Self, ForgeError> {
        let config = config::load()?;
        let discovered = plugins::discover(&config.plugin_dirs)?;
        let mut commands = BTreeMap::new();
        for command in discovered {
            if is_reserved(&command.name) {
                return Err(ForgeError::Plugin(format!(
                    "plugin command '{}' conflicts with a reserved core command",
                    command.name
                )));
            }
            if commands.insert(command.name.clone(), command).is_some() {
                return Err(ForgeError::Plugin("duplicate plugin command".into()));
            }
        }
        Ok(Self { config, commands })
    }

    pub fn run(
        &self,
        command: Option<&str>,
        args: &[String],
        options: EngineOptions,
    ) -> Result<i32, ForgeError> {
        validate_modes(command, options)?;
        let Some(command) = command else {
            self.print_help(options.output);
            return Ok(0);
        };

        match command {
            "help" => self.help(args, options.output),
            "version" => self.version(options.output),
            "config" => self.config_command(args, options.output),
            "plugin" => self.plugin_command(args, options.output),
            _ => self.run_plugin(command, args, options),
        }
    }

    fn help(&self, args: &[String], format: OutputFormat) -> Result<i32, ForgeError> {
        if let Some(command) = args.first() {
            if let Some(plugin) = self.commands.get(command) {
                println!("forge {command} - {}", plugin.description);
                println!("Usage: forge {command} [arguments]");
                return Ok(0);
            }
            if is_reserved(command) {
                println!("forge {command} - built-in command");
                return Ok(0);
            }
            return Err(ForgeError::Usage(format!("unknown command '{command}'")));
        }
        self.print_help(format);
        Ok(0)
    }

    fn print_help(&self, format: OutputFormat) {
        let mut commands = vec![
            ("config", "Inspect and update configuration"),
            ("help", "Display global or command-specific help"),
            ("plugin", "Discover and inspect extensions"),
            ("version", "Display engine and API versions"),
        ];
        for command in self.commands.values() {
            commands.push((command.name.as_str(), command.description.as_str()));
        }
        commands.sort_by_key(|command| command.0);
        if format == OutputFormat::Json {
            #[derive(Serialize)]
            struct Help<'a> {
                usage: &'a str,
                commands: Vec<(&'a str, &'a str)>,
            }
            output::print(
                format,
                &Help {
                    usage: "forge <command> [arguments] [options]",
                    commands,
                },
                "",
                "",
            );
        } else {
            println!("Usage: forge <command> [arguments] [options]\n\nCommands:");
            for (name, description) in commands {
                println!("  {name:<12} {description}");
            }
        }
    }

    fn version(&self, format: OutputFormat) -> Result<i32, ForgeError> {
        output::print(
            format,
            &VersionInfo {
                engine_version: ENGINE_VERSION,
                command_api_version: 1,
                shell_protocol_version: protocol::SHELL_PROTOCOL_VERSION,
            },
            &format!("Forge {ENGINE_VERSION} (command API 1, shell protocol 1)"),
            ENGINE_VERSION,
        );
        Ok(0)
    }

    fn config_command(&self, args: &[String], format: OutputFormat) -> Result<i32, ForgeError> {
        match args.first().map(String::as_str) {
            None | Some("show") => {
                output::print(
                    format,
                    &self.config,
                    &toml::to_string_pretty(&self.config)?,
                    &toml::to_string(&self.config)?,
                );
                Ok(0)
            }
            Some("path") => {
                let path = config::config_path()?;
                output::print(
                    format,
                    &path,
                    &path.display().to_string(),
                    &path.display().to_string(),
                );
                Ok(0)
            }
            Some("add-plugin-dir") => {
                let path = args.get(1).ok_or_else(|| {
                    ForgeError::Usage("config add-plugin-dir requires a path".into())
                })?;
                let mut config = self.config.clone();
                let path = PathBuf::from(path);
                if !config.plugin_dirs.contains(&path) {
                    config.plugin_dirs.push(path);
                    config::save(&config)?;
                }
                println!("Plugin directory configured.");
                Ok(0)
            }
            Some(value) => Err(ForgeError::Usage(format!(
                "unknown config operation '{value}'"
            ))),
        }
    }

    fn plugin_command(&self, args: &[String], format: OutputFormat) -> Result<i32, ForgeError> {
        match args.first().map(String::as_str) {
            None | Some("list") => {
                let plugins: Vec<_> = self
                    .commands
                    .values()
                    .map(|command| PluginInfo {
                        command: &command.name,
                        plugin: &command.plugin_name,
                        description: &command.description,
                    })
                    .collect();
                if plugins.is_empty() {
                    println!("No plugins discovered.");
                } else if format == OutputFormat::Json {
                    output::print(format, &plugins, "", "");
                } else {
                    for plugin in plugins {
                        println!(
                            "{:<16} {} ({})",
                            plugin.command, plugin.description, plugin.plugin
                        );
                    }
                }
                Ok(0)
            }
            Some(value) => Err(ForgeError::Usage(format!(
                "unknown plugin operation '{value}'"
            ))),
        }
    }

    fn run_plugin(
        &self,
        name: &str,
        args: &[String],
        options: EngineOptions,
    ) -> Result<i32, ForgeError> {
        let plugin = self.commands.get(name).ok_or_else(|| {
            ForgeError::Usage(format!(
                "unknown command '{name}'. Run 'forge help' for available commands."
            ))
        })?;
        if options.newshell {
            return run_in_new_shell(&plugin.executable, args);
        }
        let mut process = build_script_command(&plugin.executable)?;
        process.args(args);
        if options.capture {
            let output = process.output()?;
            print!("{}", String::from_utf8_lossy(&output.stdout));
            eprint!("{}", String::from_utf8_lossy(&output.stderr));
            return Ok(output.status.code().unwrap_or(1));
        }
        if options.detached {
            process
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()?;
            return Ok(0);
        }
        let status = process.status()?;
        Ok(status.code().unwrap_or(1))
    }
}

fn validate_modes(command: Option<&str>, options: EngineOptions) -> Result<(), ForgeError> {
    if options.detached && options.wait {
        return Err(ForgeError::Usage(
            "--detached and --wait cannot be combined".into(),
        ));
    }
    if options.capture && options.detached {
        return Err(ForgeError::Usage(
            "--capture requires a foreground process".into(),
        ));
    }
    if (options.detached || options.newshell || options.wait || options.capture)
        && command.is_none()
    {
        return Err(ForgeError::Usage(
            "execution options require a command".into(),
        ));
    }
    Ok(())
}

fn is_reserved(name: &str) -> bool {
    matches!(name, "config" | "help" | "plugin" | "version")
}

fn build_script_command(executable: &PathBuf) -> Result<Command, ForgeError> {
    let Some(extension) = executable.extension().and_then(|ext| ext.to_str()) else {
        return Ok(Command::new(executable));
    };

    match extension.to_ascii_lowercase().as_str() {
        "cmd" | "bat" => {
            let mut command = Command::new("cmd");
            command.arg("/c").arg(executable);
            Ok(command)
        }
        "ps1" => {
            let mut command = Command::new("powershell");
            command
                .arg("-NoProfile")
                .arg("-ExecutionPolicy")
                .arg("Bypass")
                .arg("-File")
                .arg(executable);
            Ok(command)
        }
        _ => Ok(Command::new(executable)),
    }
}

fn run_in_new_shell(executable: &PathBuf, args: &[String]) -> Result<i32, ForgeError> {
    let mut command = build_script_command(executable)?;
    command.args(args);
    let status = command.status()?;
    Ok(status.code().unwrap_or(1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mode_validation_rejects_conflicts() {
        let options = EngineOptions {
            output: OutputFormat::Human,
            detached: true,
            newshell: false,
            wait: true,
            capture: false,
        };
        assert!(validate_modes(Some("x"), options).is_err());
    }
}
