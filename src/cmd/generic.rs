// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use rand::Rng;

use crate::cmd::parse::{ParseCommandError, Parser};
use crate::cmd::Command;

#[derive(Debug, Clone)]
pub enum GenericCommand {
    DbSize,
    Delete(Vec<String>),
    Exists(Vec<String>),
    FlushDb(bool),
    RandomKey(usize),
    Rename(String, String),
    Type(String),
}

impl GenericCommand {
    pub(super) fn parse(
        cmd_name: &str,
        parser: &mut Parser,
    ) -> Result<Option<Command>, ParseCommandError> {
        let generic_cmd = match cmd_name {
            "dbsize" => Self::DbSize,
            "del" => {
                let keys = parser.remaining_strings()?;
                Self::Delete(keys)
            }
            "exists" => {
                let keys = parser.remaining_strings()?;
                Self::Exists(keys)
            }
            "randomkey" => {
                let mut rng = rand::thread_rng();
                let random_index = rng.gen::<usize>();
                Self::RandomKey(random_index)
            }
            "rename" => {
                let key = parser.next_string()?;
                let new_key = parser.next_string()?;
                Self::Rename(key, new_key)
            }
            "type" => {
                let key = parser.next_string()?;
                Self::Type(key)
            }
            "flushdb" | "flushall" => {
                let is_sync: bool = parser
                    .try_next_string()?
                    .map_or(true, |sync_option| sync_option.eq_ignore_ascii_case("sync"));
                Self::FlushDb(is_sync)
            }
            _ => return Ok(None),
        };
        Ok(Some(Command::Generic(generic_cmd)))
    }
}
