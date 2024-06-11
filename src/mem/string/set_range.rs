// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use bytes::{BufMut, Bytes};

use crate::cmd::reply_frame::ReplyFrame;
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
pub fn set_range(db: &mut Db, key: String, offset: isize, value: Bytes) -> ReplyFrame {
    if offset < 0 {
        return ReplyFrame::ConstError("offset is out of range");
    }
    let offset_usize = offset as usize;

    if let Some(old_value) = db.get_mut(&key) {
        let old_value = match old_value {
            MemObject::Str(s) => match s {
                StrObject::Integer(_int) => todo!(),
                StrObject::Vec(vec) => vec,
            },
            _ => return ReplyFrame::wrong_type_err(),
        };
        return if value.is_empty() {
            ReplyFrame::Usize(old_value.len())
        } else {
            if !check_string_length(offset_usize, value.len()) {
                return ReplyFrame::ConstErrorWithErr(STRING_TOO_LONG_ERR);
            }
            // FIXME(Shaohua): merge two parts of vector
            old_value.put_slice(&value);
            ReplyFrame::Usize(old_value.len())
        };
    } else {
        if value.is_empty() {
            return ReplyFrame::zero();
        }

        if !check_string_length(offset_usize, value.len()) {
            return ReplyFrame::ConstErrorWithErr(STRING_TOO_LONG_ERR);
        }

        let mut s = StrObject::with_length(offset_usize);
        s.append(&value);
        let len = s.len();
        db.insert(key, MemObject::Str(s));
        ReplyFrame::Usize(len)
    }
}
