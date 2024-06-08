// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use std::io::{Cursor, Write};
use std::num::TryFromIntError;
use std::string::FromUtf8Error;

use atoi::atoi;
use bytes::{Buf, BufMut, Bytes, BytesMut};

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
    pub fn into_bytes(self) -> Bytes {
        match self {
            Self::Array(arr) => {
                let mut bytes = BytesMut::new();
                bytes.put_u8(b'*');
                let len = arr.len();
                #[allow(clippy::cast_possible_wrap)]
                Self::write_i64(&mut bytes, len as i64);

                for frame in arr {
                    bytes.put(frame.into_bytes());
                }

                bytes.freeze()
            }
            Self::Bulk(val) => {
                let len = val.len();
                let mut bytes = BytesMut::new();
                bytes.put_u8(b'$');
                #[allow(clippy::cast_possible_wrap)]
                Self::write_i64(&mut bytes, len as i64);
                bytes.put(val);
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
            Self::Integer(num) => {
                let mut bytes = BytesMut::new();
                bytes.put_u8(b':');
                Self::write_i64(&mut bytes, num);
                bytes.freeze()
            }
            Self::Null => Bytes::from("$-1\r\n"),
            Self::Simple(s) => {
                let mut bytes = BytesMut::new();
                bytes.put_u8(b'+');
                bytes.put(Bytes::from(s));
                bytes.put_slice(b"\r\n");
                bytes.freeze()
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
        let frame_type = Self::get_u8(cursor)?;

        match frame_type {
            b'+' | b'-' => {
                let _ = Self::get_line(cursor)?;
                Ok(())
            }
            b':' => {
                let _ = Self::get_i64(cursor)?;
                Ok(())
            }
            b'$' => {
                let len = usize::try_from(Self::get_i64(cursor)?)?;
                Self::skip(cursor, len + 2)
            }
            b'*' => {
                let len = Self::get_i64(cursor)?;
                for _ in 0..len {
                    Self::check_msg(cursor)?;
                }
                Ok(())
            }
            frame_type => Err(ParsingFrameError::InvalidFrameType(frame_type)),
        }
    }

    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self, ParsingFrameError> {
        match Self::get_u8(cursor)? {
            b'+' => {
                let line = Self::get_line(cursor)?;
                let s: String = String::from_utf8(line.to_vec())?;
                Ok(Self::Simple(s))
            }
            b'-' => {
                let line = Self::get_line(cursor)?;
                let s = String::from_utf8(line.to_vec())?;
                Ok(Self::Error(s))
            }
            b':' => {
                let val = Self::get_i64(cursor)?;
                Ok(Self::Integer(val))
            }
            b'$' => {
                if b'-' == Self::peek_u8(cursor)? {
                    let line = Self::get_line(cursor)?;
                    if line == b"-1" {
                        Ok(Self::Null)
                    } else {
                        Err(ParsingFrameError::InvalidFrameFormat)
                    }
                } else {
                    let len = usize::try_from(Self::get_i64(cursor)?)?;
                    // data + '\r\n'
                    let n = len + 2;
                    if cursor.remaining() < n {
                        return Err(ParsingFrameError::Incomplete);
                    }

                    let data = Bytes::copy_from_slice(&cursor.chunk()[..len]);
                    cursor.advance(n);
                    Ok(Self::Bulk(data))
                }
            }
            b'*' => {
                let len = usize::try_from(Self::get_i64(cursor)?)?;
                let mut arr = Vec::with_capacity(len);
                for _ in 0..len {
                    arr.push(Self::parse(cursor)?);
                }
                // log::info!("frame arr: {arr:?}");

                Ok(Self::Array(arr))
            }
            _ => unimplemented!(),
        }
    }

    #[allow(dead_code)]
    fn peek_u8(cursor: &Cursor<&[u8]>) -> Result<u8, ParsingFrameError> {
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
        atoi::<i64>(line).ok_or(ParsingFrameError::InvalidFrameFormat)
    }

    /// Read one line from message.
    fn get_line<'a>(cursor: &mut Cursor<&'a [u8]>) -> Result<&'a [u8], ParsingFrameError> {
        let left = usize::try_from(cursor.position())?;
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
        Self::InvalidFrameFormat
    }
}

impl From<TryFromIntError> for ParsingFrameError {
    fn from(err: TryFromIntError) -> Self {
        log::warn!("Failed to parse int value from frame, err: {err:?}");
        Self::InvalidFrameFormat
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::Frame;

    #[test]
    fn test_parse_frame() {
        let s = "2a320d0a24330d0a6765740d0a24340d0a6e616d650d0a";
        let bytes: Vec<u8> = (0..s.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
            .collect();
        let mut cursor = Cursor::new(&bytes[..]);
        let ret = Frame::check_msg(&mut cursor);
        assert!(ret.is_ok());
        let _len = cursor.position() as usize;
        cursor.set_position(0);
        let ret = Frame::parse(&mut cursor);
        assert!(ret.is_ok());
    }
}
