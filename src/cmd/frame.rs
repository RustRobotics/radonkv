// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use std::io::Cursor;
use std::string::FromUtf8Error;

use atoi::atoi;
use bytes::{Buf, Bytes};

#[derive(Debug, Clone)]
pub enum Frame {
    Array(Vec<Frame>),
    Bulk(Bytes),
    Error(String),
    Integer(i64),
    Null,
    Simple(String),
}

#[derive(Debug)]
pub enum ParsingFrameError {
    ArrayExpected,
    Incomplete,
    InvalidFrameType(u8),
    InvalidFrameFormat,
}

impl Frame {
    #[must_use]
    #[inline]
    pub fn new_array() -> Self {
        Self::Array(vec![])
    }

    pub fn push_bulk(&mut self, bytes: Bytes) -> Result<(), ParsingFrameError> {
        if let Self::Array(vec) = self {
            vec.push(Self::Bulk(bytes));
            Ok(())
        } else {
            Err(ParsingFrameError::ArrayExpected)
        }
    }

    pub fn push_i64(&mut self, num: i64) -> Result<(), ParsingFrameError> {
        if let Self::Array(vec) = self {
            vec.push(Self::Integer(num));
            Ok(())
        } else {
            Err(ParsingFrameError::ArrayExpected)
        }
    }

    pub fn check_msg(cursor: &mut Cursor<&[u8]>) -> Result<(), ParsingFrameError> {
        // Read first byte and check its type.
        match Self::get_u8(cursor)? {
            b'+' | b'-' => {
                let _ = Self::get_line(cursor)?;
                Ok(())
            }
            b':' => {
                let _ = Self::get_i64(cursor)?;
                Ok(())
            }
            b'$' => {
                // TODO(Shaohua):
                Ok(())
            }
            b'*' => {
                let _len = Self::get_i64(cursor)?;
                // TODO(Shaohua): Check array
                Ok(())
            }
            frame_type => Err(ParsingFrameError::InvalidFrameType(frame_type))
        }
    }

    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self, ParsingFrameError> {
        match Self::get_u8(cursor)? {
            b'+' => {
                let line = Self::get_line(cursor)?;
                let s: String = String::from_utf8(line.to_vec())?;
                Ok(Frame::Simple(s))
            }
            b'-' => {
                let line = Self::get_line(cursor)?;
                let s = String::from_utf8(line.to_vec())?;
                Ok(Frame::Error(s))
            }
            b':' => {
                let val = Self::get_i64(cursor)?;
                Ok(Frame::Integer(val))
            }
            b'$' => {
                if b'-' == Self::peek_u8(cursor)? {
                    let line = Self::get_line(cursor)?;
                    if line == b"-1" {
                        Ok(Frame::Null)
                    } else {
                        Err(ParsingFrameError::InvalidFrameFormat)
                    }
                } else {
                    // TODO(Shaohua): Fix cast error
                    let len: usize = Self::get_i64(cursor)? as usize;
                    // data + '\r\n'
                    let n = len + 2;
                    if cursor.remaining() < n {
                        return Err(ParsingFrameError::Incomplete);
                    }

                    let data = Bytes::copy_from_slice(&cursor.chunk()[..len]);
                    cursor.advance(n);
                    Ok(Frame::Bulk(data))
                }
            }
            b'*' => {
                // TODO(Shaohua): Check cast overflow
                let len = Self::get_i64(cursor)? as usize;
                let mut arr = Vec::with_capacity(len);
                for _ in 0..len {
                    arr.push(Self::parse(cursor)?);
                }

                Ok(Frame::Array(arr))
            }
            _ => unimplemented!(),
        }
    }

    #[allow(dead_code)]
    fn peek_u8(cursor: &mut Cursor<&[u8]>) -> Result<u8, ParsingFrameError> {
        if cursor.has_remaining() {
            Ok(cursor.chunk()[0])
        } else {
            Err(ParsingFrameError::Incomplete)
        }
    }

    fn skip(cursor: &mut Cursor<&[u8]>, n: usize) -> Result<(), ParsingFrameError> {
        if cursor.remaining() >= n {
            cursor.advance(n);
            Ok(())
        } else {
            Err(ParsingFrameError::Incomplete)
        }
    }

    /// Read one byte from buffer.
    fn get_u8(cursor: &mut Cursor<&[u8]>) -> Result<u8, ParsingFrameError> {
        if cursor.has_remaining() {
            Ok(cursor.get_u8())
        } else {
            Err(ParsingFrameError::Incomplete)
        }
    }

    fn get_i64(cursor: &mut Cursor<&[u8]>) -> Result<i64, ParsingFrameError> {
        let line = Self::get_line(cursor)?;
        atoi::<i64>(line).ok_or_else(|| ParsingFrameError::InvalidFrameFormat)
    }

    /// Read one line from message.
    fn get_line<'a>(cursor: &mut Cursor<&'a [u8]>) -> Result<&'a [u8], ParsingFrameError> {
        let left = cursor.position() as usize;
        let right = cursor.get_ref().len() - 1;
        for i in left..right {
            if cursor.get_ref()[i] == b'\r' && cursor.get_ref()[i + 1] == b'\n' {
                cursor.set_position((i + 2) as u64);
                return Ok(&cursor.get_ref()[left..i]);
            }
        }

        Err(ParsingFrameError::Incomplete)
    }
}

impl From<FromUtf8Error> for ParsingFrameError {
    fn from(_err: FromUtf8Error) -> Self {
        ParsingFrameError::InvalidFrameFormat
    }
}