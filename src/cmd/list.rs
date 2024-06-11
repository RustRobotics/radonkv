// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::Command;
use crate::cmd::parse::{ParseCommandError, Parser};

#[derive(Debug, Clone)]
pub enum ListCommand {
    Index(String, isize),
    Len(String),
    PushBack(String, Vec<Vec<u8>>),
    PushBackExist(String, Vec<Vec<u8>>),
    PushFront(String, Vec<Vec<u8>>),
    PushFrontExist(String, Vec<Vec<u8>>),
    PopBack(String, Option<usize>),
    PopFront(String, Option<usize>),
    Range(String, isize, isize),
}

impl ListCommand {
    pub(super) fn parse(
        cmd_name: &str,
        parser: &mut Parser,
    ) -> Result<Option<Command>, ParseCommandError> {
        let list_cmd = match cmd_name {
            "lindex" => {
                let key = parser.next_string()?;
                let index = parser.next_isize()?;
                Self::Index(key, index)
            }
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
            "lpushx" => {
                let key = parser.next_string()?;
                let values = parser.remaining()?;
                Self::PushFrontExist(key, values)
            }
            "rpop" => {
                let key = parser.next_string()?;
                let count = parser.try_next_usize()?;
                Self::PopBack(key, count)
            }
            "rpush" => {
                let key = parser.next_string()?;
                let values = parser.remaining()?;
                Self::PushBack(key, values)
            }
            "rpushx" => {
                let key = parser.next_string()?;
                let values = parser.remaining()?;
                Self::PushBackExist(key, values)
            }
            "lrange" => {
                let key = parser.next_string()?;
                let start = parser.next_isize()?;
                let end = parser.next_isize()?;
                Self::Range(key, start, end)
            }
            _ => return Ok(None),
        };
        Ok(Some(Command::List(list_cmd)))
    }
}
