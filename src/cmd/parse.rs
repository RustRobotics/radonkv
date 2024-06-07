// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use std::vec::IntoIter;

use bytes::Bytes;

use crate::cmd::Command;
use crate::cmd::frame::Frame;
use crate::cmd::list::ListCommand;
use crate::cmd::string::StringCommand;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ParsingCommandError {
    CommandNotFound,
    InvalidParameter,
    ProtocolError,
}

impl TryFrom<Frame> for Command {
    type Error = ParsingCommandError;

    fn try_from(frame: Frame) -> Result<Self, Self::Error> {
        let arr: Vec<Frame> = match frame {
            Frame::Array(arr) => arr,
            frame => {
                log::warn!("Invalid frame, expected array, got: {frame:?}");
                return Err(ParsingCommandError::ProtocolError);
            }
        };

        let mut parser = Parser {
            iter: arr.into_iter(),
        };
        let cmd_name = parser.next_string()?.to_ascii_lowercase();
        // TODO(Shaohua): Add a command hash map.
        let mut command: Option<Self> = StringCommand::parse(&cmd_name, &mut parser)?;
        if command.is_none() {
            command = ListCommand::parse(&cmd_name, &parser)?;
        }
        if command.is_none() {
            log::warn!("Command not found: {cmd_name}");
        }
        command.ok_or(ParsingCommandError::CommandNotFound)
    }
}

pub struct Parser {
    iter: IntoIter<Frame>,
}

impl Parser {
    pub fn next(&mut self) -> Result<Frame, ParsingCommandError> {
        self.iter
            .next()
            .ok_or(ParsingCommandError::InvalidParameter)
    }

    pub fn next_string(&mut self) -> Result<String, ParsingCommandError> {
        match self.next()? {
            Frame::Simple(s) => Ok(s),
            Frame::Bulk(bytes) => std::str::from_utf8(&bytes[..])
                .map(std::string::ToString::to_string)
                .map_err(|err| {
                    log::warn!("Failed to parse string, got err: {err:?}");
                    ParsingCommandError::InvalidParameter
                }),
            frame => {
                log::warn!("Protocol error, expected simple or bulk frame, got: {frame:?}");
                Err(ParsingCommandError::ProtocolError)
            }
        }
    }

    pub fn next_bytes(&mut self) -> Result<Bytes, ParsingCommandError> {
        match self.next()? {
            Frame::Simple(s) => Ok(Bytes::from(s)),
            Frame::Bulk(bytes) => Ok(bytes),
            frame => {
                log::warn!("Protocol error, expected simple or bulk frame, got: {frame:?}");
                Err(ParsingCommandError::ProtocolError)
            }
        }
    }
}
