// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::Command;
use crate::cmd::parse::{ParseCommandError, Parser};

#[derive(Debug, Clone)]
pub enum ServerManagementCommand {
    Shutdown,
    Time,
}

impl ServerManagementCommand {
    pub fn parse(
        cmd_name: &str,
        _parser: &mut Parser,
    ) -> Result<Option<Command>, ParseCommandError> {
        let server_cmd = match cmd_name {
            "shutdown" => Self::Shutdown,
            "time" => Self::Time,
            _ => return Ok(None),
        };
        Ok(Some(Command::ServerManagement(server_cmd)))
    }
}
