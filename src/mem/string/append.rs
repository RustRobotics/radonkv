// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

#![allow(clippy::cast_possible_wrap)]

use std::collections::hash_map::Entry;

use bytes::Bytes;

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::{Db, MemObject};
use crate::mem::string::StrObject;

/// If key already exists and is a string, this command appends the value at the end of the string.
/// If key does not exist it is created and set as an empty string,
/// so APPEND will be similar to SET in this special case.
///
/// Returns new length of string.
pub fn append(db: &mut Db, key: String, value: Bytes) -> ReplyFrame {
    match db.entry(key) {
        Entry::Occupied(mut occupied) => match occupied.get_mut() {
            MemObject::Str(old_str) => {
                old_str.append(&value);
                ReplyFrame::Usize(old_str.len())
            }
            MemObject::List(_) => {
                ReplyFrame::wrong_type_err()
            }
        }
        Entry::Vacant(vacant) => {
            let len = value.len();
            vacant.insert(StrObject::from_bytes(value));
            ReplyFrame::Usize(len)
        }
    }
}