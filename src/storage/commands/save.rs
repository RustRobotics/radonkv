// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::reply_frame::ReplyFrame;

/// The `SAVE` commands performs a synchronous save of the dataset producing a point in time snapshot
/// of all the data inside the server instance, in the form of an RDB file.
///
/// You almost never want to call `SAVE` in production environments where it will
/// block all the other clients. Instead, the `BGSAVE` is usually used.
/// However, in case of issues preventing Redis to create the background saving child
/// (for instance errors in the fork(2) system call), the `SAVE` command can be a good last resort
/// to perform the dump of the latest dataset.
///
/// Reply:
/// - Simple string reply: OK.
pub fn save() -> ReplyFrame {
    // TODO(Shaohua): Save to RDB file.
    log::info!("[storage] TODO: save to RDB file.");
    ReplyFrame::ok()
}
