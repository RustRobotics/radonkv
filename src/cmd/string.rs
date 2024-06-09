// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use bytes::Bytes;

use crate::cmd::Command;
use crate::cmd::parse::{Parser, ParsingCommandError};

#[derive(Debug, Clone)]
pub enum StringCommand {
    Append(String, Bytes),
    Get(String),
    GetDel(String),
    GetSet(String, Bytes),
    Set(String, Bytes),
    StrLen(String),
}

impl StringCommand {
    pub fn parse(
        cmd_name: &str,
        parser: &mut Parser,
    ) -> Result<Option<Command>, ParsingCommandError> {
        let str_cmd = match cmd_name {
            "append" => {
                let key = parser.next_string()?;
                let value = parser.next_bytes()?;
                Self::Append(key, value)
            }
            "get" => {
                let key = parser.next_string()?;
                Self::Get(key)
            }
            "getdel" => {
                let key = parser.next_string()?;
                Self::GetDel(key)
            }
            "getset" => {
                let key = parser.next_string()?;
                let value = parser.next_bytes()?;
                Self::GetSet(key, value)
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
