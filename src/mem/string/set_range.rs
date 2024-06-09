// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use bytes::{BufMut, Bytes};

use crate::cmd::frame::Frame;
use crate::cmd::frame_consts::FrameConst;
use crate::mem::db::{Db, MemObject};
use crate::mem::string::consts::STRING_TOO_LONG_ERR;
use crate::mem::string::StrObject;
use crate::mem::util::check_string_length;

/// Overwrites part of the string stored at key, starting at the specified offset,
/// for the entire length of value.
///
/// If the offset is larger than the current length of the string at key,
/// the string is padded with zero-bytes to make offset fit.
/// Non-existing keys are considered as empty strings, so this command will make sure
/// it holds a string large enough to be able to set value at offset.
pub fn set_range(db: &mut Db, key: String, offset: isize, value: Bytes) -> Frame {
    if offset < 0 {
        return FrameConst::from_err("offset is out of range");
    }
    let offset_usize = offset as usize;

    if let Some(old_value) = db.get_mut(&key) {
        let old_value = match old_value {
            MemObject::Str(s) => match s {
                StrObject::Integer(_int) => todo!(),
                StrObject::Vec(vec) => {
                    vec
                }
            }
            _ => return Frame::wrong_type_err(),
        };
        return if value.is_empty() {
            Frame::Integer(old_value.len() as i64)
        } else {
            if !check_string_length(offset_usize, value.len()) {
                return FrameConst::from_str(STRING_TOO_LONG_ERR);
            }
            old_value.put_slice(&value);
            Frame::Integer(old_value.len() as i64)
        };
    } else {
        if value.is_empty() {
            return Frame::zero();
        }

        if !check_string_length(offset_usize, value.len()) {
            return FrameConst::from_str(STRING_TOO_LONG_ERR);
        }

        let mut s = StrObject::with_length(offset_usize);
        s.append(&value);
        let len = s.len();
        db.insert(key, MemObject::Str(s));
        Frame::Integer(len as i64)
    }
}