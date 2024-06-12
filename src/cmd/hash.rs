// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::Command;
use crate::cmd::parse::{ParseCommandError, Parser};

#[derive(Debug, Clone)]
pub enum HashCommand {
    Len(String),
}

impl HashCommand {
    pub(super) fn parse(
        cmd_name: &str,
        parser: &mut Parser,
    ) -> Result<Option<Command>, ParseCommandError> {
        let list_cmd = match cmd_name {
            "hlen" => {
                let key = parser.next_string()?;
                Self::Len(key)
            }
            _ => return Ok(None),
        };
        Ok(Some(Command::Hash(list_cmd)))
    }
}
