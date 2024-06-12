// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::Db;

/// Returns if key exists.
///
/// The user should be aware that if the same existing key is mentioned in the arguments multiple times,
/// it will be counted multiple times.
/// So if `somekey` exists, `EXISTS somekey somekey` will return 2.
pub fn exists(db: &Db, key: &str, extra_keys: Option<Vec<String>>) -> ReplyFrame {
    let mut count = 0;
    if db.contains_key(key) {
        count += 1;
    }
    if let Some(extra_keys) = extra_keys {
        count += extra_keys
            .iter()
            .filter(|key| db.contains_key(key.as_str()))
            .count();
    }
    ReplyFrame::Usize(count)
}
