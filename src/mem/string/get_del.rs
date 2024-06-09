// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::frame::Frame;
use crate::mem::db::{Db, MemObject};

pub fn get_del(db: &mut Db, key: &String) -> Frame {
    match db.get(key) {
        Some(MemObject::Str(value)) => {
            let frame = Frame::Bulk(value.to_bytes());
            db.remove(key);
            frame
        }
        Some(_other) => Frame::wrong_type_err(),
        None => Frame::null(),
    }
}