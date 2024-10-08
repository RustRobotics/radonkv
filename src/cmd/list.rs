// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::parse::{ParseCommandError, Parser};
use crate::cmd::Command;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum RelativePosition {
    Before,
    After,
}

impl TryFrom<String> for RelativePosition {
    type Error = ParseCommandError;

    fn try_from(mut value: String) -> Result<Self, Self::Error> {
        value.make_ascii_lowercase();
        if value == "before" {
            Ok(Self::Before)
        } else if value == "after" {
            Ok(Self::After)
        } else {
            Err(ParseCommandError::InvalidParameter)
        }
    }
}

#[derive(Debug, Clone)]
pub enum ListCommand {
    Index(String, isize),
    Insert(String, RelativePosition, Vec<Vec<u8>>),
    Len(String),
    PushBack(String, Vec<Vec<u8>>),
    PushBackExist(String, Vec<Vec<u8>>),
    PushFront(String, Vec<Vec<u8>>),
    PushFrontExist(String, Vec<Vec<u8>>),
    PopBack(String, Option<usize>),
    PopFront(String, Option<usize>),
    Range(String, isize, isize),
    Remove(String, isize, Vec<u8>),
    Set(String, isize, Vec<u8>),
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
            "linsert" => {
                let key = parser.next_string()?;
                let pos_str = parser.next_string()?;
                let position = RelativePosition::try_from(pos_str)?;
                let pivot = parser.next_bytes()?;
                let element = parser.next_bytes()?;
                let pair = vec![pivot, element];
                Self::Insert(key, position, pair)
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
            "lrem" => {
                let key = parser.next_string()?;
                let count = parser.next_isize()?;
                let element = parser.next_bytes()?;
                Self::Remove(key, count, element)
            }
            "lset" => {
                let key = parser.next_string()?;
                let index = parser.next_isize()?;
                let element = parser.next_bytes()?;
                Self::Set(key, index, element)
            }
            _ => return Ok(None),
        };
        Ok(Some(Command::List(list_cmd)))
    }
}

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use crate::cmd::list::ListCommand;

    #[test]
    fn test_list_command() {
        assert_eq!(size_of::<ListCommand>(), 64);
    }
}
