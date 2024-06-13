// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::Command;
use crate::cmd::parse::{ParseCommandError, Parser};

#[derive(Debug, Clone)]
pub enum SetCommand {
    Add(String, Vec<Vec<u8>>),
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
            _ => return Ok(None),
        };
        Ok(Some(Command::Set(set_cmd)))
    }
}
