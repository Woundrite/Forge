use clap::ValueEnum;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum OutputFormat {
    Human,
    Plain,
    Json,
}

pub fn print<T: Serialize>(format: OutputFormat, value: &T, human: &str, plain: &str) {
    match format {
        OutputFormat::Human => println!("{human}"),
        OutputFormat::Plain => println!("{plain}"),
        OutputFormat::Json => println!(
            "{}",
            serde_json::to_string_pretty(value).unwrap_or_else(|_| "{}".to_owned())
        ),
    }
}
