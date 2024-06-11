// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use std::num::{ParseFloatError, ParseIntError};
use std::vec::IntoIter;

use crate::cmd::Command;
use crate::cmd::frame::Frame;
use crate::cmd::generic::GenericCommand;
use crate::cmd::list::ListCommand;
use crate::cmd::string::StringCommand;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ParseCommandError {
    CommandNotFound,
    InvalidParameter,
    ProtocolError,
}

impl From<ParseIntError> for ParseCommandError {
    fn from(_err: ParseIntError) -> Self {
        // TODO(Shaohua): Inherit from thiserror
        Self::InvalidParameter
    }
}

impl From<ParseFloatError> for ParseCommandError {
    fn from(_err: ParseFloatError) -> Self {
        // TODO(Shaohua):
        Self::InvalidParameter
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

        let mut parser = Parser {
            iter: arr.into_iter(),
        };
        let cmd_name = parser.next_string()?.to_ascii_lowercase();
        // TODO(Shaohua): Add a command hash map.
        let mut command: Option<Self> = StringCommand::parse(&cmd_name, &mut parser)?;
        if command.is_none() {
            command = ListCommand::parse(&cmd_name, &mut parser)?;
        }
        if command.is_none() {
            command = GenericCommand::parse(&cmd_name, &mut parser)?;
        }
        if command.is_none() {
            log::warn!("Command not found: {cmd_name}");
        }
        command.ok_or(ParseCommandError::CommandNotFound)
    }
}

pub struct Parser {
    iter: IntoIter<Frame>,
}

impl Parser {
    pub fn next(&mut self) -> Result<Frame, ParseCommandError> {
        self.iter.next().ok_or(ParseCommandError::InvalidParameter)
    }

    pub fn remaining(&mut self) -> Result<Option<Vec<Vec<u8>>>, ParseCommandError> {
        let mut list = Vec::new();
        while let Some(frame) = self.iter.next() {
            match frame {
                Frame::Bulk(frame) => list.push(frame.to_vec()),
                frame => {
                    log::warn!("Protocol error, expected bulk frame, got: {frame:?}");
                    return Err(ParseCommandError::ProtocolError);
                }
            }
        }
        if list.is_empty() {
            Ok(None)
        } else {
            Ok(Some(list))
        }
    }

    pub fn remaining_strings(&mut self) -> Result<Vec<String>, ParseCommandError> {
        let mut list = Vec::new();
        while let Some(frame) = self.iter.next() {
            match frame {
                Frame::Bulk(bytes) => {
                    let s = std::str::from_utf8(&bytes[..])
                        .map(ToString::to_string)
                        .map_err(|err| {
                            log::warn!("Failed to parse string, got err: {err:?}");
                            ParseCommandError::InvalidParameter
                        })?;
                    list.push(s);
                }
                frame => {
                    log::warn!("Protocol error, expected bulk frame, got: {frame:?}");
                    return Err(ParseCommandError::ProtocolError);
                }
            }
        }
        Ok(list)
    }

    pub fn next_string(&mut self) -> Result<String, ParseCommandError> {
        match self.next()? {
            Frame::Simple(s) => Ok(s),
            Frame::Bulk(bytes) => std::str::from_utf8(&bytes[..])
                .map(std::string::ToString::to_string)
                .map_err(|err| {
                    log::warn!("Failed to parse string, got err: {err:?}");
                    ParseCommandError::InvalidParameter
                }),
            frame => {
                log::warn!("Protocol error, expected simple or bulk frame, got: {frame:?}");
                Err(ParseCommandError::ProtocolError)
            }
        }
    }

    pub fn next_i32(&mut self) -> Result<i32, ParseCommandError> {
        match self.next()? {
            Frame::Simple(s) => Ok(s.parse::<i32>()?),
            // TODO(Shaohua): Convert ParseCommandError as complex enum
            Frame::Bulk(bytes) => {
                let s = std::str::from_utf8(&bytes[..]).map_err(|err| {
                    log::warn!("Failed to parse string, got err: {err:?}");
                    ParseCommandError::InvalidParameter
                })?;
                Ok(s.parse::<i32>()?)
            }
            frame => {
                log::warn!("Protocol error, expected simple or bulk frame, got: {frame:?}");
                Err(ParseCommandError::ProtocolError)
            }
        }
    }

    pub fn next_isize(&mut self) -> Result<isize, ParseCommandError> {
        match self.next()? {
            Frame::Simple(s) => Ok(s.parse::<isize>()?),
            Frame::Bulk(bytes) => {
                let s = std::str::from_utf8(&bytes[..]).map_err(|err| {
                    log::warn!("Failed to parse string, got err: {err:?}");
                    ParseCommandError::InvalidParameter
                })?;
                Ok(s.parse::<isize>()?)
            }
            frame => {
                log::warn!("Protocol error, expected simple or bulk frame, got: {frame:?}");
                Err(ParseCommandError::ProtocolError)
            }
        }
    }

    pub fn try_next_usize(&mut self) -> Result<Option<usize>, ParseCommandError> {
        match self.iter.next() {
            None => Ok(None),
            Some(Frame::Simple(s)) => {
                let num = s.parse::<usize>()?;
                Ok(Some(num))
            }
            Some(Frame::Bulk(bytes)) => {
                let s = std::str::from_utf8(&bytes[..]).map_err(|err| {
                    log::warn!("Failed to parse string, got err: {err:?}");
                    ParseCommandError::InvalidParameter
                })?;
                let num = s.parse::<usize>()?;
                Ok(Some(num))
            }
            Some(frame) => {
                log::warn!("Protocol error, expected simple or bulk frame, got: {frame:?}");
                Err(ParseCommandError::ProtocolError)
            }
        }
    }

    pub fn next_i64(&mut self) -> Result<i64, ParseCommandError> {
        match self.next()? {
            Frame::Simple(s) => Ok(s.parse::<i64>()?),
            Frame::Bulk(bytes) => {
                let s = std::str::from_utf8(&bytes[..]).map_err(|err| {
                    log::warn!("Failed to parse string, got err: {err:?}");
                    ParseCommandError::InvalidParameter
                })?;
                Ok(s.parse::<i64>()?)
            }
            frame => {
                log::warn!("Protocol error, expected simple or bulk frame, got: {frame:?}");
                Err(ParseCommandError::ProtocolError)
            }
        }
    }

    pub fn next_f64(&mut self) -> Result<f64, ParseCommandError> {
        match self.next()? {
            Frame::Simple(s) => Ok(s.parse::<f64>()?),
            Frame::Bulk(bytes) => {
                let s = std::str::from_utf8(&bytes[..]).map_err(|err| {
                    log::warn!("Failed to parse string, got err: {err:?}");
                    ParseCommandError::InvalidParameter
                })?;
                Ok(s.parse::<f64>()?)
            }
            frame => {
                log::warn!("Protocol error, expected simple or bulk frame, got: {frame:?}");
                Err(ParseCommandError::ProtocolError)
            }
        }
    }

    // TODO(Shaohua): Add next_f128()

    pub fn next_bytes(&mut self) -> Result<Vec<u8>, ParseCommandError> {
        match self.next()? {
            Frame::Simple(s) => Ok(s.into_bytes()),
            Frame::Bulk(bytes) => Ok(bytes.to_vec()),
            frame => {
                log::warn!("Protocol error, expected simple or bulk frame, got: {frame:?}");
                Err(ParseCommandError::ProtocolError)
            }
        }
    }
}
