// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::Db;

/// Return a random key from the currently selected database.
pub fn random_key(db: &Db, random_index: usize) -> ReplyFrame {
    if !db.is_empty() {
        let index = random_index % db.len();
        if let Some(key) = db.keys().nth(index) {
            return ReplyFrame::Bulk(key.as_bytes().to_vec());
        }
    }

    ReplyFrame::EmptyBulk
}