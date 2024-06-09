// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

//! This module keeps frame constants.

use bytes::Bytes;

use crate::cmd::frame::Frame;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum FrameConst {
    EmptyBulk,
    EmptyArray,
    Str(&'static str),
    // Add ERR prefix to str.
    Err(&'static str),
}

impl FrameConst {
    #[must_use]
    #[inline]
    pub(super) fn to_real_frame(self) -> Frame {
        match self {
            Self::EmptyBulk => Frame::Bulk(Bytes::new()),
            Self::EmptyArray => Frame::Array(vec![]),
            Self::Str(s) => Frame::Simple(s.to_owned()),
            Self::Err(err) => {
                let s: String = [ERR, err].concat();
                Frame::Simple(s)
            }
        }
    }

    #[must_use]
    #[inline]
    pub const fn from_str(s: &'static str) -> Frame {
        Frame::Const(Self::Str(s))
    }

    #[must_use]
    #[inline]
    pub const fn from_err(s: &'static str) -> Frame {
        Frame::Const(Self::Err(s))
    }
}

impl Frame {
    #[must_use]
    #[inline]
    pub const fn null() -> Self {
        Self::Null
    }

    #[must_use]
    #[inline]
    pub const fn ok() -> Self {
        FrameConst::from_str(OK)
    }

    #[must_use]
    #[inline]
    pub const fn empty_bulk() -> Self {
        Self::Const(FrameConst::EmptyBulk)
    }

    #[must_use]
    #[inline]
    pub const fn empty_array() -> Self {
        Self::Const(FrameConst::EmptyArray)
    }

    #[must_use]
    #[inline]
    pub const fn pong() -> Self {
        FrameConst::from_str(PONG)
    }

    #[must_use]
    #[inline]
    pub const fn queued() -> Self {
        FrameConst::from_str(QUEUED)
    }

    #[must_use]
    #[inline]
    pub const fn wrong_type_err() -> Self {
        FrameConst::from_str(WRONG_TYPE_ERR)
    }
}

pub const OK: &str = "Ok";
pub const PONG: &str = "PONG";
pub const QUEUED: &str = "QUEUED";

// Shared command error responses
pub const WRONG_TYPE_ERR: &str = "WRONGTYPE Operation against a key holding the wrong kind of value";
pub const ERR: &str = "ERR";
pub const NO_KEY_ERR: &str = "ERR no such key";
pub const SYNTAX_ERR: &str = "ERR syntax error";
pub const SAME_OBJECT_ERR: &str = "ERR source and destination objects are the same";
pub const OUT_OF_RANGE_ERR: &str = "ERR index out of range";
pub const NO_SCRIPT_ERR: &str = "NOSCRIPT No matching script. Please use EVAL.";
pub const LOADING_ERR: &str = "LOADING Server is loading the dataset in memory";
pub const SLOW_EVAL_ERR: &str = "BUSY Server is busy running a script. You can only call SCRIPT KILL or SHUTDOWN NOSAVE.";
pub const SLOW_SCRIPT_ERR: &str = "BUSY Redis is busy running a script. You can only call FUNCTION KILL or SHUTDOWN NOSAVE.";
pub const SLOW_MODULE_ERR: &str = "BUSY Redis is busy running a module command.";
pub const MASTER_DOWN_ERR: &str = "MASTERDOWN Link with MASTER is down and replica-serve-stale-data is set to 'no'.";
pub const BG_SAVE_ERR: &str = "MISCONF Redis is configured to save RDB snapshots, but it's currently unable to persist to disk. Commands that may modify the data set are disabled, because this instance is configured to report errors during writes if RDB snapshotting fails (stop-writes-on-bgsave-error option). Please check the Redis logs for details about the RDB error.";
pub const RO_SLAVE_ERR: &str = "READONLY You can't write against a read only replica.";
pub const NO_AUTH_ERR: &str = "NOAUTH Authentication required.";
pub const OOM_ERR: &str = "OOM command not allowed when used memory > 'maxmemory'.";
pub const EXEC_ABORT_ERR: &str = "EXECABORT Transaction discarded because of previous errors.";
pub const NO_REPLICAS_ERR: &str = "NOREPLICAS Not enough good replicas to write.";
pub const BUSY_KEY_ERR: &str = "BUSYKEY Target key name already exists.";
