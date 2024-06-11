// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::{Db, MemObject};

/// Returns the length of the list stored at key.
///
/// If key does not exist, it is interpreted as an empty list and 0 is returned.
/// An error is returned when the value stored at key is not a list.
pub fn len(db: &Db, key: &str) -> ReplyFrame {
    match db.get(key) {
        Some(MemObject::List(list)) => ReplyFrame::Usize(list.len()),
        Some(_other) => ReplyFrame::wrong_type_err(),
        None => ReplyFrame::zero(),
    }
}