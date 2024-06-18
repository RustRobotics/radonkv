// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::reply_frame::ReplyFrame;

/// Instruct server to start an Append Only File rewrite process.
///
/// The rewrite will create a small optimized version of the current `Append Only File`.
///
/// If `BGREWRITEAOF` fails, no data gets lost as the old AOF will be untouched.
///
/// The rewrite will be only triggered by server if there is not already
/// a background process doing persistence.
///
/// ## RESP2 Reply
///
/// Simple string reply: a simple string reply indicating that the rewriting started
/// or is about to start ASAP when the call is executed with success.
///
/// The command may reply with an error in certain cases, as documented above.
///
/// ## RESP3 Reply
///
/// Bulk string reply: a simple string reply indicating that the rewriting started
/// or is about to start ASAP when the call is executed with success.
///
/// The command may reply with an error in certain cases, as documented above.
pub fn background_write_aof() -> ReplyFrame {
    log::info!("TODO(Shaohua): [storage] impl background_write_aof()");
    ReplyFrame::ConstSimple("Background append only file rewriting started")
}
