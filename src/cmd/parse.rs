// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use std::num::{ParseFloatError, ParseIntError};
use std::vec::IntoIter;

use stdext::function_name;

use crate::cmd::frame::Frame;

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

pub struct Parser {
    iter: IntoIter<Frame>,
}

impl Parser {
    #[must_use]
    #[inline]
    pub const fn new(iter: IntoIter<Frame>) -> Self {
        Self { iter }
    }

    pub fn next(&mut self) -> Result<Frame, ParseCommandError> {
        self.iter.next().ok_or(ParseCommandError::InvalidParameter)
    }

    pub fn remaining(&mut self) -> Result<Vec<Vec<u8>>, ParseCommandError> {
        let mut list = Vec::new();
        for frame in self.iter.as_ref() {
            match frame {
                Frame::Bulk(frame) => list.push(frame.to_vec()),
                frame => {
                    log::warn!("Protocol error, expected bulk frame, got: {frame:?}");
                    return Err(ParseCommandError::ProtocolError);
                }
            }
        }
        if list.is_empty() {
            Err(ParseCommandError::InvalidParameter)
        } else {
            Ok(list)
        }
    }

    pub fn remaining_strings(&mut self) -> Result<Vec<String>, ParseCommandError> {
        let mut list = Vec::new();
        for frame in self.iter.as_ref() {
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
        if list.is_empty() {
            Err(ParseCommandError::ProtocolError)
        } else {
            Ok(list)
        }
    }

    pub fn remaining_pairs(&mut self) -> Result<Vec<(String, Vec<u8>)>, ParseCommandError> {
        let remains = self.remaining()?;
        log::debug!("{} remains: {remains:?}", function_name!());
        let mut list: Vec<(String, Vec<u8>)> = Vec::with_capacity(remains.len());
        if remains.len() % 2 != 0 {
            return Err(ParseCommandError::InvalidParameter);
        }
        for i in (0..remains.len()).step_by(2) {
            let s = std::str::from_utf8(&remains[i])
                .map(ToString::to_string)
                .map_err(|err| {
                    log::warn!("Failed to parse string, got err: {err:?}");
                    ParseCommandError::InvalidParameter
                })?;
            list.push((s, remains[i + 1].clone()));
        }
        Ok(list)
    }

    pub fn next_string(&mut self) -> Result<String, ParseCommandError> {
        self.try_next_string()?
            .ok_or(ParseCommandError::InvalidParameter)
    }

    pub fn try_next_string(&mut self) -> Result<Option<String>, ParseCommandError> {
        if let Some(frame) = self.iter.next() {
            match frame {
                Frame::Simple(s) => Ok(Some(s)),
                Frame::Bulk(bytes) => {
                    let s = std::str::from_utf8(&bytes[..])
                        .map(ToString::to_string)
                        .map_err(|err| {
                            log::warn!("Failed to parse string, got err: {err:?}");
                            ParseCommandError::InvalidParameter
                        })?;
                    Ok(Some(s))
                }
                frame => {
                    log::warn!("Protocol error, expected simple or bulk frame, got: {frame:?}");
                    Err(ParseCommandError::ProtocolError)
                }
            }
        } else {
            Ok(None)
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

    pub fn try_next_isize(&mut self) -> Result<Option<isize>, ParseCommandError> {
        match self.iter.next() {
            Some(Frame::Simple(s)) => Ok(Some(s.parse::<isize>()?)),
            Some(Frame::Bulk(bytes)) => {
                let s = std::str::from_utf8(&bytes[..]).map_err(|err| {
                    log::warn!("Failed to parse string, got err: {err:?}");
                    ParseCommandError::InvalidParameter
                })?;
                Ok(Some(s.parse::<isize>()?))
            }
            Some(frame) => {
                log::warn!("Protocol error, expected simple or bulk frame, got: {frame:?}");
                Err(ParseCommandError::ProtocolError)
            }
            None => Ok(None),
        }
    }

    #[inline]
    pub fn next_isize(&mut self) -> Result<isize, ParseCommandError> {
        self.try_next_isize()?
            .ok_or(ParseCommandError::InvalidParameter)
    }

    #[inline]
    pub fn next_usize(&mut self) -> Result<usize, ParseCommandError> {
        self.try_next_usize()?
            .ok_or(ParseCommandError::InvalidParameter)
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
