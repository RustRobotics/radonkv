// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use bytes::Bytes;

use crate::cmd::Command;
use crate::cmd::parse::{ParseCommandError, Parser};

#[derive(Debug, Clone)]
pub enum StringCommand {
    Append(String, Bytes),
    Get(String),
    GetDel(String),
    GetSet(String, Bytes),
    Set(String, Bytes),
    StrLen(String),
    SubStr(String, i64, i64),
}

impl StringCommand {
    pub fn parse(
        cmd_name: &str,
        parser: &mut Parser,
    ) -> Result<Option<Command>, ParseCommandError> {
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
            "substr" => {
                let key = parser.next_string()?;
                let start = parser.next_i64()?;
                let end = parser.next_i64()?;
                Self::SubStr(key, start, end)
            }
            _ => return Ok(None),
        };

        Ok(Some(Command::Str(str_cmd)))
    }
}
