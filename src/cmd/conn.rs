// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::Command;
use crate::cmd::parse::{ParseCommandError, Parser};

#[derive(Debug, Clone)]
pub enum ConnectManagementCommand {
    Ping(Option<String>),
    Echo(String),
}

impl ConnectManagementCommand {
    pub(super) fn parse(
        cmd_name: &str,
        parser: &mut Parser,
    ) -> Result<Option<Command>, ParseCommandError> {
        let conn_cmd = match cmd_name {
            "ping" => {
                let message = parser.try_next_string()?;
                Self::Ping(message)
            }
            "echo" => {
                let message = parser.next_string()?;
                Self::Echo(message)
            }
            _ => return Ok(None),
        };
        Ok(Some(Command::ConnManagement(conn_cmd)))
    }
}
