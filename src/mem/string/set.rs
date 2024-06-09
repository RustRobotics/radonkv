// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use bytes::Bytes;

use crate::cmd::frame::Frame;
use crate::mem::db::{Db, MemObject};
use crate::mem::string::StrObject;

pub fn set(db: &mut Db, key: String, value: Bytes) -> Frame {
    db.insert(key, MemObject::Str(StrObject::from_bytes(value)));
    Frame::ok()
}