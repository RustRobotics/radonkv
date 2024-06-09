// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use std::fmt;
use std::io;

use tokio::sync::mpsc;

use crate::cmd::frame::ParseFrameError;

#[derive(Debug, Clone)]
pub struct Error {
    kind: ErrorKind,
    message: String,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ErrorKind {
    // Startup errors.
    ConfigError,
    LoggerError,

    // System errors.
    IoError,
    KernelError,
    ChannelError,
    SocketError,

    // Client errors
    FrameError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error kind: {:?}, msg: {}", self.kind, self.message)
    }
}

impl std::error::Error for Error {}

impl Error {
    #[must_use]
    #[inline]
    pub const fn from_string(kind: ErrorKind, message: String) -> Self {
        Self { kind, message }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self {
            kind: ErrorKind::IoError,
            message: format!("{err:?}"),
        }
    }
}

impl From<ParseFrameError> for Error {
    fn from(err: ParseFrameError) -> Self {
        Self {
            kind: ErrorKind::FrameError,
            message: format!("{err:?}"),
        }
    }
}

impl<T> From<mpsc::error::SendError<T>> for Error {
    fn from(err: mpsc::error::SendError<T>) -> Self {
        Self::from_string(ErrorKind::ChannelError, format!("channel error: {err:?}"))
    }
}
