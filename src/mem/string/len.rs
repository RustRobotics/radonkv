// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::frame::Frame;
use crate::mem::db::{Db, MemObject};

#[allow(clippy::cast_possible_wrap)]
pub fn len(db: &Db, key: &str) -> Frame {
    match db.get(key) {
        Some(MemObject::Str(value)) => Frame::Integer(value.len() as i64),
        Some(_other) => Frame::Error(
            "Object type mismatch, expected string".to_owned(),
        ),
        None => Frame::Integer(0),
    }
}