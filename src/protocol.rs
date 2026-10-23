use serde::{Deserialize, Serialize};

pub const SHELL_PROTOCOL_VERSION: u32 = 1;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum ShellAction {
    ChangeDirectory { path: String },
    SetEnvironment { name: String, value: String },
    RemoveEnvironment { name: String },
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ShellResponse {
    pub protocol_version: u32,
    pub action: ShellAction,
    pub diagnostic: Option<String>,
}

impl ShellResponse {
    pub fn change_directory(path: String) -> Self {
        Self {
            protocol_version: SHELL_PROTOCOL_VERSION,
            action: ShellAction::ChangeDirectory { path },
            diagnostic: None,
        }
    }
}

pub fn validate(response: &ShellResponse) -> Result<(), &'static str> {
    if response.protocol_version != SHELL_PROTOCOL_VERSION {
        return Err("unsupported shell protocol version");
    }
    match &response.action {
        ShellAction::ChangeDirectory { path } if path.is_empty() => {
            Err("directory path cannot be empty")
        }
        ShellAction::SetEnvironment { name, .. } | ShellAction::RemoveEnvironment { name }
            if name.is_empty() || name.contains(['=', '\0']) =>
        {
            Err("invalid environment variable name")
        }
        _ => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shell_response_round_trips_as_json() {
        let response = ShellResponse::change_directory("C:\\work".into());
        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(
            serde_json::from_str::<ShellResponse>(&json).unwrap(),
            response
        );
    }
}
