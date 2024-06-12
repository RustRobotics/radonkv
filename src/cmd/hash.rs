// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::Command;
use crate::cmd::parse::{ParseCommandError, Parser};

pub type ExtraValues = Option<Vec<(String, Vec<u8>)>>;

#[derive(Debug, Clone)]
pub enum HashCommand {
    Del(String, String, Option<Vec<String>>),
    Exists(String, String),
    Get(String, String),
    GetAll(String),
    Keys(String),
    Len(String),
    Set(String, String, Vec<u8>, ExtraValues),
    StrLen(String, String),
    Values(String),
}

impl HashCommand {
    pub(super) fn parse(
        cmd_name: &str,
        parser: &mut Parser,
    ) -> Result<Option<Command>, ParseCommandError> {
        let list_cmd = match cmd_name {
            "hdel" => {
                let key = parser.next_string()?;
                let field = parser.next_string()?;
                let extra_fields = parser.remaining_strings()?;
                Self::Del(key, field, extra_fields)
            }
            "hexists" => {
                let key = parser.next_string()?;
                let field = parser.next_string()?;
                Self::Exists(key, field)
            }
            "hget" => {
                let key = parser.next_string()?;
                let field = parser.next_string()?;
                Self::Get(key, field)
            }
            "hgetall" => {
                let key = parser.next_string()?;
                Self::GetAll(key)
            }
            "hkeys" => {
                let key = parser.next_string()?;
                Self::Keys(key)
            }
            "hlen" => {
                let key = parser.next_string()?;
                Self::Len(key)
            }
            "hset" => {
                let key = parser.next_string()?;
                let field = parser.next_string()?;
                let value = parser.next_bytes()?;
                let extra_values = parser.remaining_pairs()?;
                Self::Set(key, field, value, extra_values)
            }
            "hstrlen" => {
                let key = parser.next_string()?;
                let field = parser.next_string()?;
                Self::StrLen(key, field)
            }
            "hvals" => {
                let key = parser.next_string()?;
                Self::Values(key)
            }
            _ => return Ok(None),
        };
        Ok(Some(Command::Hash(list_cmd)))
    }
}
