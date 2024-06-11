// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::{Db, MemObject};

#[allow(clippy::cast_possible_wrap)]
pub fn len(db: &Db, key: &str) -> ReplyFrame {
    match db.get(key) {
        Some(MemObject::Str(value)) => ReplyFrame::Usize(value.len()),
        Some(_other) => ReplyFrame::ConstError(
            "Object type mismatch, expected string"
        ),
        None => ReplyFrame::Usize(0),
    }
}