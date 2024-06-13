// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use std::collections::hash_map::Entry;
use std::mem;

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::bitmap::BitmapObject;
use crate::mem::db::{Db, MemObject};

pub fn from_bytes(db: &mut Db, key: String, value: &[u8]) -> ReplyFrame {
    match db.entry(key) {
        Entry::Occupied(mut occupied) => match occupied.get_mut() {
            MemObject::Bitmap(old_bitmap) => {
                let mut new_bitmap = BitmapObject::from_bytes(value);
                mem::swap(old_bitmap, &mut new_bitmap);
                ReplyFrame::ok()
            }
            _ => ReplyFrame::wrong_type_err(),
        },
        Entry::Vacant(vacant) => {
            let new_bitmap = BitmapObject::from_bytes(value);
            vacant.insert(MemObject::Bitmap(new_bitmap));
            ReplyFrame::ok()
        }
    }
}
