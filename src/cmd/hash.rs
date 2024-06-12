// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::Command;
use crate::cmd::parse::{ParseCommandError, Parser};

pub type ExtraValues = Option<Vec<(String, Vec<u8>)>>;

#[derive(Debug, Clone)]
pub enum HashCommand {
    Get(String, String),
    GetAll(String),
    Len(String),
    Set(String, String, Vec<u8>, ExtraValues),
}

impl HashCommand {
    pub(super) fn parse(
        cmd_name: &str,
        parser: &mut Parser,
    ) -> Result<Option<Command>, ParseCommandError> {
        let list_cmd = match cmd_name {
            "hget" => {
                let key = parser.next_string()?;
                let field = parser.next_string()?;
                Self::Get(key, field)
            }
            "hgetall" => {
                let key = parser.next_string()?;
                Self::GetAll(key)
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
            _ => return Ok(None),
        };
        Ok(Some(Command::Hash(list_cmd)))
    }
}
