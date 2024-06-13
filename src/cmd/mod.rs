// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::conn::ConnectManagementCommand;
use crate::cmd::frame::Frame;
use crate::cmd::generic::GenericCommand;
use crate::cmd::hash::HashCommand;
use crate::cmd::hyper_log_log::HyperLogLogCommand;
use crate::cmd::list::ListCommand;
use crate::cmd::parse::{ParseCommandError, Parser};
use crate::cmd::string::StringCommand;

pub mod conn;
pub mod frame;
pub mod generic;
pub mod hash;
pub mod hyper_log_log;
pub mod list;
mod parse;
pub mod reply_frame;
pub mod string;

#[derive(Debug, Clone)]
pub enum Command {
    Str(StringCommand),
    List(ListCommand),
    Hash(HashCommand),
    HyperLogLog(HyperLogLogCommand),
    Generic(GenericCommand),
    ConnManagement(ConnectManagementCommand),
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub enum CommandCategory {
    #[default]
    Mem,
    System,
    Cluster,
    Storage,
    // Handle commands in session module.
    Session,
}

impl Command {
    #[must_use]
    pub const fn category(&self) -> CommandCategory {
        match self {
            Self::Str(_)
            | Self::List(_)
            | Self::Generic(_)
            | Self::Hash(_)
            | Self::HyperLogLog(_) => CommandCategory::Mem,
            Self::ConnManagement(_) => CommandCategory::Session,
        }
    }

    #[must_use]
    #[inline]
    pub fn is_mem(&self) -> bool {
        self.category() == CommandCategory::Mem
    }
}

impl TryFrom<Frame> for Command {
    type Error = ParseCommandError;

    fn try_from(frame: Frame) -> Result<Self, Self::Error> {
        let arr: Vec<Frame> = match frame {
            Frame::Array(arr) => arr,
            frame => {
                log::warn!("Invalid frame, expected array, got: {frame:?}");
                return Err(ParseCommandError::ProtocolError);
            }
        };

        let mut parser = Parser::new(arr.into_iter());
        let cmd_name = parser.next_string()?.to_ascii_lowercase();
        // TODO(Shaohua): Add a command hash map.
        let mut command: Option<Self> = StringCommand::parse(&cmd_name, &mut parser)?;
        if command.is_none() {
            command = ListCommand::parse(&cmd_name, &mut parser)?;
        }
        if command.is_none() {
            command = HashCommand::parse(&cmd_name, &mut parser)?;
        }
        if command.is_none() {
            command = HyperLogLogCommand::parse(&cmd_name, &mut parser)?;
        }
        if command.is_none() {
            command = GenericCommand::parse(&cmd_name, &mut parser)?;
        }
        if command.is_none() {
            command = ConnectManagementCommand::parse(&cmd_name, &mut parser)?;
        }
        if command.is_none() {
            log::warn!("Command not found: {cmd_name}");
        }
        command.ok_or(ParseCommandError::CommandNotFound)
    }
}

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use crate::cmd::Command;

    #[test]
    fn test_command() {
        assert_eq!(size_of::<Command>(), 72);
    }
}
