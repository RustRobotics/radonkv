// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::Command;
use crate::cmd::parse::{ParseCommandError, Parser};

#[derive(Debug, Clone)]
pub enum HyperLogLogCommand {
    Count(String, Vec<String>),
}

impl HyperLogLogCommand {
    pub(super) fn parse(
        cmd_name: &str,
        parser: &mut Parser,
    ) -> Result<Option<Command>, ParseCommandError> {
        let hyper_cmd = match cmd_name {
            "pfcount" => {
                let primary_key = parser.next_string()?;
                let extra_keys = parser.remaining_strings()?;
                Self::Count(primary_key, extra_keys)
            }
            _ => return Ok(None),
        };
        Ok(Some(Command::HyperLogLog(hyper_cmd)))
    }
}
