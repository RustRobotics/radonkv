// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::{Db, MemObject};
use crate::mem::string::StrObject;

/// Atomically sets key to value and returns the old value stored at key.
///
/// Returns an error when key exists but does not hold a string value.
/// Any previous time to live associated with the key is discarded on successful SET operation.
pub fn get_set(db: &mut Db, key: String, value: Vec<u8>) -> ReplyFrame {
    match db.get(&key) {
        Some(MemObject::Str(old_value)) => {
            let frame = old_value.to_bulk();
            db.insert(key, StrObject::from_bytes(value));
            frame
        }
        Some(_other) => ReplyFrame::wrong_type_err(),
        None => {
            db.insert(key, StrObject::from_bytes(value));
            ReplyFrame::Null
        }
    }
}