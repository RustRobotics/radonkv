// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::parse::{ParseCommandError, Parser};
use crate::cmd::Command;

#[derive(Debug, Clone)]
pub enum HyperLogLogCommand {
    Add(String, Vec<String>),
    Count(Vec<String>),
    Merge(String, Vec<String>),
}

impl HyperLogLogCommand {
    pub(super) fn parse(
        cmd_name: &str,
        parser: &mut Parser,
    ) -> Result<Option<Command>, ParseCommandError> {
        let hyper_cmd = match cmd_name {
            "pfadd" => {
                let key = parser.next_string()?;
                let elements = parser.remaining_strings()?;
                Self::Add(key, elements)
            }
            "pfcount" => {
                let keys = parser.remaining_strings()?;
                Self::Count(keys)
            }
            "pfmerge" => {
                let dest_key = parser.next_string()?;
                let source_keys = parser.remaining_strings()?;
                Self::Merge(dest_key, source_keys)
            }
            _ => return Ok(None),
        };
        Ok(Some(Command::HyperLogLog(hyper_cmd)))
    }
}
