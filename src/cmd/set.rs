// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::parse::{ParseCommandError, Parser};
use crate::cmd::Command;

#[derive(Debug, Clone)]
pub enum SetCommand {
    Add(String, Vec<Vec<u8>>),
    Len(String),
    Members(String),
    IsMember(String, Vec<u8>),
    RandomMember(String, Option<isize>),
    Remove(String, Vec<Vec<u8>>),
    Intersect(Vec<String>),
    Union(Vec<String>),
    Diff(Vec<String>),
}

impl SetCommand {
    pub fn parse(
        cmd_name: &str,
        parser: &mut Parser,
    ) -> Result<Option<Command>, ParseCommandError> {
        let set_cmd = match cmd_name {
            "sadd" => {
                let key = parser.next_string()?;
                let members = parser.remaining()?;
                Self::Add(key, members)
            }
            "slen" | "scard" => {
                let key = parser.next_string()?;
                Self::Len(key)
            }
            "smembers" => {
                let key = parser.next_string()?;
                Self::Members(key)
            }
            "sismember" => {
                let key = parser.next_string()?;
                let member = parser.next_bytes()?;
                Self::IsMember(key, member)
            }
            "srandmember" => {
                let key = parser.next_string()?;
                let count = parser.try_next_isize()?;
                Self::RandomMember(key, count)
            }
            "srem" => {
                let key = parser.next_string()?;
                let members = parser.remaining()?;
                Self::Remove(key, members)
            }
            "sinter" => {
                let keys = parser.remaining_strings()?;
                Self::Intersect(keys)
            }
            "sunion" => {
                let keys = parser.remaining_strings()?;
                Self::Union(keys)
            }
            "sdiff" => {
                let keys = parser.remaining_strings()?;
                Self::Diff(keys)
            }
            _ => return Ok(None),
        };
        Ok(Some(Command::Set(set_cmd)))
    }
}
