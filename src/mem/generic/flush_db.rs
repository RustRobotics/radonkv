// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use std::mem;

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::Db;

/// Delete all the keys of the currently selected DB.
///
/// This command never fails.
///
/// By default, `FLUSHDB` will synchronously flush all keys from the database.
/// Setting the `lazyfree-lazy-user-flush` configuration directive to "yes"
/// changes the default flush mode to asynchronous.
///
/// Note: an asynchronous `FLUSHDB` command only deletes keys that were present
/// at the time the command was invoked.
/// Keys created during an asynchronous flush will be unaffected.
///
/// Reply:
/// - Simple string reply: OK.
pub fn flush_db(db: &mut Db, is_sync: bool) -> ReplyFrame {
    let mut new_db = Db::new();
    mem::swap(db, &mut new_db);
    if !is_sync {
        tokio::spawn(async move {
            drop(new_db);
        });
    }
    ReplyFrame::ok()
}
