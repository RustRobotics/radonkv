// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::{Db, MemObject};

/// Get the value of key and delete the key.
///
/// This command is similar to GET, except for the fact that it also deletes the key
/// on success (if and only if the key's value type is a string).
pub fn get_del(db: &mut Db, key: &String) -> ReplyFrame {
    match db.get(key) {
        Some(MemObject::Str(value)) => {
            let frame = ReplyFrame::Bulk(value.to_bytes());
            db.remove(key);
            frame
        }
        Some(_other) => ReplyFrame::wrong_type_err(),
        None => ReplyFrame::Null,
    }
}