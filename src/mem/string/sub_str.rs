// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::frame::Frame;
use crate::mem::db::{Db, MemObject};
use crate::mem::string::StrObject;
use crate::util::prune_range::slice_range_to_bytes;

/// Returns the substring of the string value stored at key,
/// determined by the offsets start and end (both are inclusive).
///
/// Negative offsets can be used in order to provide an offset starting from the end of the string.
/// So -1 means the last character, -2 the penultimate and so forth.
///
/// The function handles out of range requests by limiting the resulting range to the actual length of the string.
pub fn sub_str(db: &Db, key: &str, start: i64, end: i64) -> Frame {
    match db.get(key) {
        Some(MemObject::Str(value)) => match value {
            StrObject::Integer(_) => todo!(),
            StrObject::Vec(vec) => {
                let bytes = slice_range_to_bytes(vec, start, end);
                Frame::Bulk(bytes)
            }
        }
        Some(_other) => Frame::wrong_type_err(),
        None => Frame::null(),
    }
}