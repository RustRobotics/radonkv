// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use bytes::Bytes;

use crate::cmd::parse::{Parser, ParsingCommandError};
use crate::cmd::Command;

#[derive(Debug, Clone)]
pub enum StringCommand {
    Get(String),
    Set(String, Bytes),
    StrLen(String),
}

impl StringCommand {
    pub fn parse(
        cmd_name: &str,
        parser: &mut Parser,
    ) -> Result<Option<Command>, ParsingCommandError> {
        let str_cmd = match cmd_name {
            "get" => {
                let key = parser.next_string()?;
                Self::Get(key)
            }
            "set" => {
                let key = parser.next_string()?;
                let value = parser.next_bytes()?;
                Self::Set(key, value)
            }
            "strlen" => {
                let key = parser.next_string()?;
                Self::StrLen(key)
            }
            _ => return Ok(None),
        };

        Ok(Some(Command::Str(str_cmd)))
    }
}
