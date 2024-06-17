// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use std::cmp::Ordering;
use std::io::{Cursor, Write};

use bytes::{BufMut, BytesMut};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ReplyFrame {
    /// # Simple strings
    ///
    /// Simple strings are encoded as a plus (+) character, followed by a string.
    ///
    /// status values, simple string
    /// The string mustn't contain a CR (\r) or LF (\n) character and is terminated by CRLF (i.e., \r\n).
    ///
    /// Simple strings transmit short, non-binary strings with minimal overhead.
    /// For example, many Redis commands reply with just "OK" on success.
    /// The encoding of this Simple String is the following 5 bytes:
    /// ```txt
    /// +OK\r\n
    /// ```
    Simple(String),
    ConstSimple(&'static str),

    /// # Simple errors
    ///
    /// RESP has specific data types for errors.
    /// Simple errors, or simply just errors, are similar to simple strings, but their first character
    /// is the minus (-) character. The difference between simple strings and errors in RESP is that
    /// clients should treat errors as exceptions, whereas the string encoded in the error type
    /// is the error message itself.
    ///
    /// The basic format is:
    /// ```txt
    /// -Error message\r\n
    /// ```
    Error(String),
    ConstError(&'static str),
    /// constant values which represents errors, with ERR as prefix string.
    ConstErrorWithErr(&'static str),

    /// # Arrays
    ///
    /// Clients send commands to the server as RESP arrays.
    ///
    /// Similarly, some commands that return collections of elements use arrays as their replies.
    /// An example is the `LRANGE` command that returns elements of a list.
    ///
    /// RESP Arrays' encoding uses the following format:
    /// ```txt
    /// *<number-of-elements>\r\n<element-1>...<element-n>
    /// ```
    ///
    /// Based on the following rules:
    /// - An asterisk `*` as the first byte.
    /// - One or more decimal digits (0..9) as the number of elements in the array as an unsigned, base-10 value.
    /// - The CRLF terminator.
    /// - An additional RESP type for every element of the array.
    ///
    /// So an empty Array is just the following:
    /// ```txt
    /// *0\r\n
    /// ```
    ///
    /// Whereas the encoding of an array consisting of the two bulk strings "hello" and "world" is:
    /// ```txt
    /// *2\r\n$5\r\nhello\r\n$5\r\nworld\r\n
    /// ```
    ///
    /// Arrays can contain mixed data types. For instance, the following encoding is of a list
    /// of four integers and a bulk string:
    /// ```txt
    /// *5\r\n
    /// :1\r\n
    /// :2\r\n
    /// :3\r\n
    /// :4\r\n
    /// $5\r\n
    /// hello\r\n
    /// ```
    ///
    /// ## Null arrays
    /// Whereas RESP3 has a dedicated data type for null values, RESP2 has no such type.
    ///
    /// Instead, due to historical reasons, the representation of null values in RESP2
    /// is via predetermined forms of the Bulk Strings and arrays types.
    ///
    /// Null arrays exist as an alternative way of representing a null value.
    /// For instance, when the `BLPOP` command times out, it returns a null array.
    ///
    /// The encoding of a null array is that of an array with the length of -1, i.e.:
    /// ```txt
    /// *-1\r\n
    /// ```
    ///
    /// ## Null elements in arrays
    ///
    /// Single elements of an array may be null bulk string.
    ///
    /// This is used in replies to signal that these elements are missing and not empty strings.
    /// This can happen, for example, with the `SORT` command when used with the `GET` pattern option
    /// if the specified key is missing.
    ///
    /// Here's an example of an array reply containing a null element:
    /// ```txt
    /// *3\r\n
    /// $5\r\n
    /// hello\r\n
    /// $-1\r\n
    /// $5\r\n
    /// world\r\n
    /// ```
    Array(Vec<ReplyFrame>),
    EmptyArray,
    // TODO(Shaohua): Add NullArray
    //NullArray,
    /// # Bulk strings
    ///
    /// A bulk string represents a single binary string.
    ///
    /// The string can be of any size, but by default, server limits it to 512 MB
    /// (see the proto-max-bulk-len configuration directive).
    ///
    /// RESP encodes bulk strings in the following way:
    /// ```txt
    /// $<length>\r\n<data>\r\n
    /// ```
    ///
    /// Based on these rules:
    /// - The dollar sign `$` as the first byte.
    /// - One or more decimal digits (0..9) as the string's length, in bytes, as an unsigned, base-10 value.
    /// - The CRLF terminator.
    /// - The data.
    /// - A final CRLF.
    ///
    /// The empty string's encoding is:
    /// ```txt
    /// $0\r\n\r\n
    /// ```
    ///
    /// ## Null bulk strings
    ///
    /// Whereas RESP3 has a dedicated data type for null values, RESP2 has no such type.
    /// Instead, due to historical reasons, the representation of null values in RESP2
    /// is via predetermined forms of the bulk strings and arrays types.
    ///
    /// It is encoded as a bulk string with the length of negative one (-1), like so:
    /// ```txt
    /// $-1\r\n
    /// ```
    Bulk(Vec<u8>),
    /// Empty bulk string.
    EmptyBulk,
    /// Nil bulk string.
    // TODO(Shaohua): Support Null data type in RESP3
    Null,

    /// Integers
    ///
    /// This type is a CRLF-terminated string that represents a signed, base-10, 64-bit integer.
    ///
    /// RESP encodes integers in the following way:
    /// ```txt
    /// :[<+|->]<value>\r\n
    /// ```
    ///
    /// Rules are:
    /// - The colon `:` as the first byte.
    /// - An optional plus `+` or minus `-` as the sign.
    /// - One or more decimal digits (0..9) as the integer's unsigned, base-10 value.
    /// - The CRLF terminator.
    I64(i64),
    I32(i32),
    Usize(usize),
    // TODO(Shaohua): Define double
    Double(f64),
    // TODO(Shaohua): Define boolean
    // TODO(Shaohua): Define Null type
}

impl ReplyFrame {
    pub fn to_bytes(&self, bytes: &mut BytesMut) {
        match self {
            Self::Simple(s) => {
                bytes.put_u8(b'+');
                bytes.put(s.as_bytes());
                bytes.put_slice(b"\r\n");
            }
            Self::ConstSimple(s) => {
                bytes.put_u8(b'+');
                bytes.put(s.as_bytes());
                bytes.put_slice(b"\r\n");
            }
            Self::Error(err) => {
                bytes.put_u8(b'-');
                bytes.put(err.as_bytes());
                bytes.put_slice(b"\r\n");
            }
            Self::ConstError(err) => {
                bytes.put_u8(b'-');
                bytes.put(err.as_bytes());
                bytes.put_slice(b"\r\n");
            }
            Self::ConstErrorWithErr(err) => {
                bytes.put_u8(b'-');
                bytes.put(ERR.as_bytes());
                bytes.put(err.as_bytes());
                bytes.put_slice(b"\r\n");
            }

            Self::Array(arr) => {
                bytes.put_u8(b'*');
                let len = arr.len();
                #[allow(clippy::cast_possible_wrap)]
                Self::write_usize(bytes, len);

                for frame in arr {
                    frame.to_bytes(bytes);
                }
            }

            Self::EmptyArray => {
                todo!()
            }

            Self::Bulk(val) => {
                let len = val.len();
                bytes.put_u8(b'$');
                #[allow(clippy::cast_possible_wrap)]
                Self::write_usize(bytes, len);
                bytes.put_slice(val);
                bytes.put_slice(b"\r\n");
            }
            Self::EmptyBulk => {
                let len: i64 = 0;
                bytes.put_u8(b'$');
                #[allow(clippy::cast_possible_wrap)]
                Self::write_i64(bytes, len);
                bytes.put_slice(b"\r\n");
            }
            Self::Null => {
                bytes.put_slice(b"$-1\r\n");
            }

            Self::I64(num) => {
                bytes.put_u8(b':');
                Self::write_i64(bytes, *num);
            }
            Self::I32(num) => {
                bytes.put_u8(b':');
                Self::write_i64(bytes, i64::from(*num));
            }
            Self::Usize(num) => {
                bytes.put_u8(b':');
                Self::write_usize(bytes, *num);
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

    #[allow(dead_code)]
    #[allow(clippy::single_match)]
    fn sort_array(&mut self) {
        match self {
            Self::Array(array) => array.sort_unstable_by(|a, b| match (a, b) {
                (Self::Bulk(a), Self::Bulk(b)) => a.cmp(b),
                (Self::Simple(a), Self::Simple(b)) => a.cmp(b),
                _ => Ordering::Equal,
            }),
            _ => (),
        }
    }
}

impl ReplyFrame {
    #[must_use]
    #[inline]
    pub const fn null() -> Self {
        Self::Null
    }

    #[must_use]
    #[inline]
    pub const fn bulk(value: Vec<u8>) -> Self {
        Self::Bulk(value)
    }

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
    pub const fn from_bool(is_set: bool) -> Self {
        Self::Usize(if is_set { 1 } else { 0 })
    }

    #[must_use]
    #[inline]
    pub const fn minus_one() -> Self {
        Self::I32(-1)
    }

    #[must_use]
    #[inline]
    pub const fn ok() -> Self {
        Self::ConstSimple(OK)
    }

    #[must_use]
    #[inline]
    pub const fn pong() -> Self {
        Self::ConstSimple(PONG)
    }

    #[must_use]
    #[inline]
    pub const fn queued() -> Self {
        Self::ConstSimple(QUEUED)
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

    #[must_use]
    #[inline]
    pub const fn out_of_range_err() -> Self {
        Self::ConstError(OUT_OF_RANGE_ERR)
    }

    #[must_use]
    #[inline]
    pub const fn internal_err() -> Self {
        Self::ConstError(INTERNAL_ERR)
    }
}

pub const OK: &str = "Ok";
pub const PONG: &str = "PONG";
pub const QUEUED: &str = "QUEUED";

// Shared command error responses
pub const WRONG_TYPE_ERR: &str =
    "WRONGTYPE Operation against a key holding the wrong kind of value";
pub const ERR: &str = "ERR";
pub const INVALID_COMMAND: &str = "ERR invalid command";
pub const INTERNAL_ERR: &str = "ERR Unexpected server internal error";
pub const NO_KEY_ERR: &str = "ERR no such key";
pub const SYNTAX_ERR: &str = "ERR syntax error";
pub const SAME_OBJECT_ERR: &str = "ERR source and destination objects are the same";
pub const OUT_OF_RANGE_ERR: &str = "ERR index out of range";
pub const NO_SCRIPT_ERR: &str = "NOSCRIPT No matching script. Please use EVAL.";
pub const LOADING_ERR: &str = "LOADING Server is loading the dataset in memory";
pub const SLOW_EVAL_ERR: &str =
    "BUSY Server is busy running a script. You can only call SCRIPT KILL or SHUTDOWN NOSAVE.";
pub const SLOW_SCRIPT_ERR: &str =
    "BUSY Redis is busy running a script. You can only call FUNCTION KILL or SHUTDOWN NOSAVE.";
pub const SLOW_MODULE_ERR: &str = "BUSY Redis is busy running a module command.";
pub const MASTER_DOWN_ERR: &str =
    "MASTERDOWN Link with MASTER is down and replica-serve-stale-data is set to 'no'.";
pub const BG_SAVE_ERR: &str = "MISCONF Redis is configured to save RDB snapshots, but it's currently unable to persist to disk. Commands that may modify the data set are disabled, because this instance is configured to report errors during writes if RDB snapshotting fails (stop-writes-on-bgsave-error option). Please check the Redis logs for details about the RDB error.";
pub const RO_SLAVE_ERR: &str = "READONLY You can't write against a read only replica.";
pub const NO_AUTH_ERR: &str = "NOAUTH Authentication required.";
pub const OOM_ERR: &str = "OOM command not allowed when used memory > 'maxmemory'.";
pub const EXEC_ABORT_ERR: &str = "EXECABORT Transaction discarded because of previous errors.";
pub const NO_REPLICAS_ERR: &str = "NOREPLICAS Not enough good replicas to write.";
pub const BUSY_KEY_ERR: &str = "BUSYKEY Target key name already exists.";
