// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use std::collections::hash_map::Entry;
use std::mem;

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::{Db, MemObject};
use crate::mem::string::StrObject;

pub fn set(db: &mut Db, key: String, mut value: Vec<u8>) -> ReplyFrame {
    match db.entry(key) {
        Entry::Occupied(mut occupied) => match occupied.get_mut() {
            MemObject::Str(old_str) => {
                mem::swap(&mut old_str.vec, &mut value);
                ReplyFrame::ok()
            }
            _ => ReplyFrame::wrong_type_err(),
        },
        Entry::Vacant(vacant) => {
            vacant.insert(StrObject::from_bytes(value));
            ReplyFrame::ok()
        }
    }
}
