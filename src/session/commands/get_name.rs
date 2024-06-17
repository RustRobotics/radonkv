// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::reply_frame::ReplyFrame;

/// The `CLIENT GETNAME` returns the name of the current connection as
/// set by `CLIENT SETNAME`.
///
/// Since every new connection starts without an associated name,
/// if no name was assigned a null bulk reply is returned.
///
/// One of the following reply:
/// - Bulk string reply: the connection name of the current connection.
/// - Null reply: the connection name was not set.

pub fn get_name(old_name: Option<&String>) -> ReplyFrame {
    old_name.map_or(ReplyFrame::Null, |name| {
        ReplyFrame::Bulk(name.as_bytes().to_vec())
    })
}
