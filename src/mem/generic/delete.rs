// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::Db;

/// Removes the specified keys. A key is ignored if it does not exist.
pub fn delete(db: &mut Db, keys: Vec<String>) -> ReplyFrame {
    let mut count: usize = 0;
    for key in keys {
        if let Some(_value) = db.remove(&key) {
            count += 1;
        }
    }

    ReplyFrame::Usize(count)
}