// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use bytes::Bytes;

use crate::cmd::Command;
use crate::cmd::parse::{ParseCommandError, Parser};

#[derive(Debug, Clone)]
pub enum ListCommand {
    Len(String),
    PushBack(String, Vec<Bytes>),
    PushFront(String, Vec<Bytes>),
    PopFront(String, Option<usize>),
}

impl ListCommand {
    pub(super) fn parse(
        cmd_name: &str,
        parser: &mut Parser,
    ) -> Result<Option<Command>, ParseCommandError> {
        let list_cmd = match cmd_name {
            "llen" => {
                let key = parser.next_string()?;
                Self::Len(key)
            }
            "lpop" => {
                let key = parser.next_string()?;
                let count = parser.try_next_usize()?;
                Self::PopFront(key, count)
            }
            "lpush" => {
                let key = parser.next_string()?;
                let values = parser.remaining()?;
                Self::PushFront(key, values)
            }
            "rpush" => {
                let key = parser.next_string()?;
                let values = parser.remaining()?;
                Self::PushBack(key, values)
            }
            _ => return Ok(None),
        };
        Ok(Some(Command::List(list_cmd)))
    }
}
