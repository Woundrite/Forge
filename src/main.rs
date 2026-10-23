mod config;
mod engine;
mod error;
mod output;
mod plugins;
mod protocol;

use clap::Parser;
use engine::{Engine, EngineOptions};
use output::OutputFormat;
use std::process::ExitCode;

#[derive(Debug, Parser)]
#[command(
    name = "forge",
    version,
    about = "A fast, extensible command-line toolkit."
)]
struct Cli {
    #[arg(long, global = true, value_enum, default_value_t = OutputFormat::Human)]
    output: OutputFormat,

    #[arg(long, global = true)]
    detached: bool,

    #[arg(long, global = true)]
    newshell: bool,

    #[arg(long, global = true)]
    wait: bool,

    #[arg(long, global = true)]
    capture: bool,

    #[arg()]
    command: Option<String>,

    #[arg(trailing_var_arg = true)]
    args: Vec<String>,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let options = EngineOptions {
        output: cli.output,
        detached: cli.detached,
        newshell: cli.newshell,
        wait: cli.wait,
        capture: cli.capture,
    };

    match Engine::load() {
        Ok(engine) => match engine.run(cli.command.as_deref(), &cli.args, options) {
            Ok(code) => ExitCode::from(code as u8),
            Err(error) => {
                eprintln!("{error}");
                ExitCode::from(error.exit_code() as u8)
            }
        },
        Err(error) => {
            eprintln!("{error}");
            ExitCode::from(error.exit_code() as u8)
        }
    }
}
