// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::{Db, MemObject};

/// Returns the string representation of the type of the value stored at key.
///
/// The different types that can be returned are:
/// - string
/// - list
/// - set
/// - zset
/// - hash
/// - stream
pub fn get_type(db: &Db, key: &str) -> ReplyFrame {
    let obj_type = match db.get(key) {
        Some(MemObject::Str(_)) => "string",
        Some(MemObject::List(_)) => "list",
        Some(MemObject::Hash(_)) => "hash",
        Some(MemObject::Set(_)) => "set",
        // TODO(Shaohua): Returns "string" instead of "hyper"
        Some(MemObject::Hyper(_)) => "hyper",
        None => "none",
    };
    ReplyFrame::ConstStatus(obj_type)
}
