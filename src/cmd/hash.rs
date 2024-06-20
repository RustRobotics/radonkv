// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::parse::{ParseCommandError, Parser};
use crate::cmd::Command;

#[derive(Debug, Clone)]
pub enum HashCommand {
    Del(String, Vec<String>),
    Exists(String, String),
    Get(String, String),
    GetAll(String),
    Keys(String),
    Len(String),
    Set(String, Vec<(String, Vec<u8>)>),
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
                let fields = parser.remaining_strings()?;
                Self::Del(key, fields)
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
                let pairs = parser.remaining_pairs()?;
                Self::Set(key, pairs)
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

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use crate::cmd::hash::HashCommand;

    #[test]
    fn test_hash_command() {
        assert_eq!(size_of::<HashCommand>(), 56);
    }
}
