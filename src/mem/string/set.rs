// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use bytes::Bytes;

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::Db;
use crate::mem::string::StrObject;

pub fn set(db: &mut Db, key: String, value: Bytes) -> ReplyFrame {
    // TODO(Shaohua): Check type of old value.
    db.insert(key, StrObject::from_bytes(value));
    ReplyFrame::ok()
}