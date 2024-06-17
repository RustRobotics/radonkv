// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::Db;

/// Return the number of keys in the currently-selected database.
///
/// Reply:
/// - Integer reply: the number of keys in the currently-selected database.
pub fn db_size(db: &Db) -> ReplyFrame {
    ReplyFrame::Usize(db.len())
}
