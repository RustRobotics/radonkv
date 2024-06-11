// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::Command;
use crate::cmd::parse::{ParseCommandError, Parser};

#[derive(Debug, Clone)]
pub enum StringCommand {
    Append(String, Vec<u8>),
    Get(String),
    GetDel(String),
    GetRange(String, isize, isize),
    GetSet(String, Vec<u8>),
    Set(String, Vec<u8>),
    SetRange(String, isize, Vec<u8>),
    StrLen(String),
    SubStr(String, isize, isize),
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
            "getrange" => {
                let key = parser.next_string()?;
                let start = parser.next_isize()?;
                let end = parser.next_isize()?;
                Self::GetRange(key, start, end)
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
            "setrange" => {
                let key = parser.next_string()?;
                let offset = parser.next_isize()?;
                let value = parser.next_bytes()?;
                Self::SetRange(key, offset, value)
            }
            "strlen" => {
                let key = parser.next_string()?;
                Self::StrLen(key)
            }
            "substr" => {
                let key = parser.next_string()?;
                let start = parser.next_isize()?;
                let end = parser.next_isize()?;
                Self::SubStr(key, start, end)
            }
            _ => return Ok(None),
        };

        Ok(Some(Command::Str(str_cmd)))
    }
}
