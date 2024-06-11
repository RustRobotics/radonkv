// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::Db;

/// Renames key to `new_key`.
///
/// It returns an error when key does not exist.
/// If `new_key` already exists it is overwritten, when this happens
/// `RENAME` executes an implicit `DEL` operation, so if the deleted key
/// contains a very big value it may cause high latency even if RENAME itself
/// is usually a constant-time operation.
//
// In Cluster mode, both key and `new_key` must be in the same hash slot,
// meaning that in practice only keys that have the same hashtag can be reliably renamed in cluster.
pub fn rename(db: &mut Db, key: String, new_key: String) -> ReplyFrame {
    if let Some(value) = db.remove(&key) {
        db.insert(new_key, value);
        ReplyFrame::ok()
    } else {
        ReplyFrame::no_such_key()
    }
}