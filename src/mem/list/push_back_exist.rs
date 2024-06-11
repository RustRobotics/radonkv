// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use std::collections::hash_map::Entry;

use bytes::Bytes;

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::{Db, MemObject};

/// Inserts specified values at the tail of the list stored at key, only if key already exists
/// and holds a list.
///
/// In contrary to `RPUSH`, no operation will be performed when key does not yet exist.
pub fn push_back_exist(db: &mut Db, key: String, values: Vec<Bytes>) -> ReplyFrame {
    match db.entry(key) {
        Entry::Occupied(mut occupied) => match occupied.get_mut() {
            MemObject::Str(_) => ReplyFrame::wrong_type_err(),
            MemObject::List(old_list) => {
                for value in values {
                    old_list.push_back(value.to_vec());
                }
                ReplyFrame::Usize(old_list.len())
            }
        }
        Entry::Vacant(_vacant) => {
            ReplyFrame::zero()
        }
    }
}