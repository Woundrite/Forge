use thiserror::Error;

#[derive(Debug, Error)]
pub enum ForgeError {
    #[error("invalid usage: {0}")]
    Usage(String),
    #[error("configuration error: {0}")]
    Configuration(String),
    #[error("plugin error: {0}")]
    Plugin(String),
    #[error("command failed: {0}")]
    Command(String),
    #[error("internal error: {0}")]
    Internal(String),
}

impl ForgeError {
    pub fn exit_code(&self) -> i32 {
        match self {
            Self::Usage(_) => 2,
            Self::Configuration(_) | Self::Plugin(_) | Self::Command(_) => 1,
            Self::Internal(_) => 70,
        }
    }
}

impl From<std::io::Error> for ForgeError {
    fn from(value: std::io::Error) -> Self {
        Self::Command(value.to_string())
    }
}

impl From<toml::de::Error> for ForgeError {
    fn from(value: toml::de::Error) -> Self {
        Self::Configuration(value.to_string())
    }
}

impl From<toml::ser::Error> for ForgeError {
    fn from(value: toml::ser::Error) -> Self {
        Self::Configuration(value.to_string())
    }
}
