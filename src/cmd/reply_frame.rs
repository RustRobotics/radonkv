// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use std::io::{Cursor, Write};

use bytes::{BufMut, Bytes, BytesMut};

#[derive(Debug, Clone)]
pub enum ReplyFrame {
    // status values
    Status(String),
    ConstStatus(&'static str),

    // errors
    Error(String),
    ConstError(&'static str),
    // constant values which represents errors, with ERR as prefix string.
    ConstErrorWithErr(&'static str),

    // Array
    Array(Vec<ReplyFrame>),
    EmptyArray,

    // String or bytes
    Bulk(Vec<u8>),
    // Empty bulk string.
    EmptyBulk,
    // Nil bulk string.
    Null,

    // Decimal
    I64(i64),
    I32(i32),
    Usize(usize),
    Double(f64),

}

impl ReplyFrame {
    pub fn into_bytes(self) -> Bytes {
        match self {
            Self::Status(s) => {
                let mut bytes = BytesMut::new();
                bytes.put_u8(b'+');
                bytes.put(s.as_bytes());
                bytes.put_slice(b"\r\n");
                bytes.freeze()
            }
            Self::ConstStatus(s) => {
                let mut bytes = BytesMut::new();
                bytes.put_u8(b'+');
                bytes.put(s.as_bytes());
                bytes.put_slice(b"\r\n");
                bytes.freeze()
            }
            Self::Error(err) => {
                let mut bytes = BytesMut::new();
                bytes.put_u8(b'-');
                bytes.put(Bytes::from(err));
                bytes.put_slice(b"\r\n");
                bytes.freeze()
            }
            Self::ConstError(err) => {
                let mut bytes = BytesMut::new();
                bytes.put_u8(b'-');
                bytes.put(err.as_bytes());
                bytes.put_slice(b"\r\n");
                bytes.freeze()
            }
            Self::ConstErrorWithErr(err) => {
                let mut bytes = BytesMut::new();
                bytes.put_u8(b'-');
                bytes.put(ERR.as_bytes());
                bytes.put(err.as_bytes());
                bytes.put_slice(b"\r\n");
                bytes.freeze()
            }

            Self::Array(arr) => {
                let mut bytes = BytesMut::new();
                bytes.put_u8(b'*');
                let len = arr.len();
                #[allow(clippy::cast_possible_wrap)]
                Self::write_usize(&mut bytes, len);

                for frame in arr {
                    bytes.put(frame.into_bytes());
                }

                bytes.freeze()
            }

            Self::EmptyArray => {
                todo!()
            }

            Self::Bulk(val) => {
                let len = val.len();
                let mut bytes = BytesMut::new();
                bytes.put_u8(b'$');
                #[allow(clippy::cast_possible_wrap)]
                Self::write_usize(&mut bytes, len);
                bytes.put_slice(&val);
                bytes.put_slice(b"\r\n");
                bytes.freeze()
            }
            Self::EmptyBulk => {
                let len: i64 = 0;
                let mut bytes = BytesMut::new();
                bytes.put_u8(b'$');
                #[allow(clippy::cast_possible_wrap)]
                Self::write_i64(&mut bytes, len);
                bytes.put_slice(b"\r\n");
                bytes.freeze()
            }
            Self::Null => Bytes::from("$-1\r\n"),

            Self::I64(num) => {
                let mut bytes = BytesMut::new();
                bytes.put_u8(b':');
                Self::write_i64(&mut bytes, num);
                bytes.freeze()
            }
            Self::I32(num) => {
                let mut bytes = BytesMut::new();
                bytes.put_u8(b':');
                Self::write_i64(&mut bytes, num as i64);
                bytes.freeze()
            }
            Self::Usize(num) => {
                let mut bytes = BytesMut::new();
                bytes.put_u8(b':');
                Self::write_usize(&mut bytes, num);
                bytes.freeze()
            }
            Self::Double(_num) => {
                // TODO(Shaohua): Convert as bulk reply
                todo!()
            }
        }
    }

    fn write_i64(bytes: &mut BytesMut, val: i64) {
        // NOTE(Shaohua): Replace String format with stack array.
        let mut buf = [0u8; 32];
        let mut cursor = Cursor::new(&mut buf[..]);
        write!(&mut cursor, "{val}").unwrap();
        let pos = usize::try_from(cursor.position()).unwrap();
        bytes.put(&cursor.get_ref()[0..pos]);
        bytes.put_slice(b"\r\n");
    }

    fn write_usize(bytes: &mut BytesMut, val: usize) {
        // NOTE(Shaohua): Replace String format with stack array.
        let mut buf = [0u8; 32];
        let mut cursor = Cursor::new(&mut buf[..]);
        write!(&mut cursor, "{val}").unwrap();
        let pos = usize::try_from(cursor.position()).unwrap();
        bytes.put(&cursor.get_ref()[0..pos]);
        bytes.put_slice(b"\r\n");
    }
}

impl ReplyFrame {
    #[must_use]
    #[inline]
    pub const fn zero() -> Self {
        Self::Usize(0)
    }

    #[must_use]
    #[inline]
    pub const fn one() -> Self {
        Self::Usize(1)
    }

    #[must_use]
    #[inline]
    pub const fn ok() -> Self {
        Self::ConstStatus(OK)
    }

    #[must_use]
    #[inline]
    pub const fn pong() -> Self {
        Self::ConstStatus(PONG)
    }

    #[must_use]
    #[inline]
    pub const fn queued() -> Self {
        Self::ConstStatus(QUEUED)
    }

    #[must_use]
    #[inline]
    pub const fn wrong_type_err() -> Self {
        Self::ConstError(WRONG_TYPE_ERR)
    }

    #[must_use]
    #[inline]
    pub const fn invalid_command() -> Self {
        Self::ConstError(INVALID_COMMAND)
    }

    #[must_use]
    #[inline]
    pub const fn no_such_key() -> Self {
        Self::ConstError(NO_KEY_ERR)
    }
}


pub const OK: &str = "Ok";
pub const PONG: &str = "PONG";
pub const QUEUED: &str = "QUEUED";

// Shared command error responses
pub const WRONG_TYPE_ERR: &str = "WRONGTYPE Operation against a key holding the wrong kind of value";
pub const INVALID_COMMAND: &str = "ERR Invalid command";
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
