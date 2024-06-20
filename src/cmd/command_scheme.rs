// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use std::collections::HashMap;
use std::path::Path;
use std::{fs, io};

use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Clone, Deserialize)]
pub struct CommandScheme {
    pub summary: String,
    pub complexity: Option<String>,
    pub group: String,
    pub since: String,
    pub arity: i32,
    pub function: String,
    pub history: Option<Vec<CommandSchemeHistory>>,
    pub command_flags: Vec<String>,
    pub arguments: Option<Vec<CommandSchemeArgument>>,
}

pub type CommandSchemeMap = HashMap<String, CommandScheme>;

pub type CommandSchemeHistory = Vec<String>;

#[derive(Debug, Clone, Deserialize)]
pub struct CommandSchemeArgument {
    pub name: String,
    #[serde(rename(deserialize = "type", serialize = "type"))]
    pub type_: String,
    pub token: Option<String>,
    pub optional: Option<bool>,
    pub since: Option<String>,
}

#[derive(Debug, Error)]
pub enum ParseCommandSchemeError {
    #[error("Io Error")]
    IoError(#[from] io::Error),

    #[error("Parse json error")]
    DeserializeError(#[from] serde_json::Error),

    #[error("Scheme file is invalid")]
    EmptyScheme,
}

impl CommandScheme {
    pub fn parse<P: AsRef<Path>>(filepath: P) -> Result<CommandSchemeMap, ParseCommandSchemeError> {
        let json_content = fs::read_to_string(filepath.as_ref())?;
        let scheme_map: CommandSchemeMap = serde_json::from_str(&json_content)?;
        Ok(scheme_map)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::cmd::command_scheme::{CommandScheme, CommandSchemeMap};

    #[test]
    fn test_parse_scheme_by_hand() {
        let json_content = fs::read_to_string("assets/commands/strlen.json");
        assert!(json_content.is_ok());
        let json_content = json_content.unwrap();
        let scheme_map: Result<CommandSchemeMap, _> = serde_json::from_str(&json_content);
        println!("scheme map: {scheme_map:?}");
        assert!(scheme_map.is_ok());
        let scheme_map = scheme_map.unwrap();
        let scheme = scheme_map.get("STRLEN");
        assert!(scheme.is_some());
        let scheme = scheme.unwrap();
        assert_eq!(scheme.function, "strlenCommand");
    }

    #[test]
    fn test_parse() {
        let scheme_map = CommandScheme::parse("assets/commands/strlen.json");
        assert!(scheme_map.is_ok());
        let scheme_map = scheme_map.unwrap();
        let scheme = scheme_map.get("STRLEN").unwrap();
        assert_eq!(scheme.function, "strlenCommand");
    }
}
