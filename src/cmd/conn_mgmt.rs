// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::parse::{ParseCommandError, Parser};
use crate::cmd::Command;

#[derive(Debug, Clone)]
pub enum ConnectManagementCommand {
    Ping(Option<String>),
    Echo(String),
    GetId(),
    GetName(),
    SetName(String),
}

impl ConnectManagementCommand {
    pub(super) fn parse(
        cmd_name: &str,
        parser: &mut Parser,
    ) -> Result<Option<Command>, ParseCommandError> {
        let conn_cmd = match cmd_name {
            "client" => {
                let mut sub_command = parser.next_string()?;
                sub_command.make_ascii_lowercase();
                match sub_command.as_str() {
                    "id" => Self::GetId(),
                    "getname" => Self::GetName(),
                    "setname" => {
                        let new_name = parser.next_string()?;
                        Self::SetName(new_name)
                    }
                    _ => return Err(ParseCommandError::InvalidParameter),
                }
            }
            "echo" => {
                let message = parser.next_string()?;
                Self::Echo(message)
            }
            "ping" => {
                let message = parser.try_next_string()?;
                Self::Ping(message)
            }
            _ => return Ok(None),
        };
        Ok(Some(Command::ConnManagement(conn_cmd)))
    }
}
