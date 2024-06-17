// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::reply_frame::ReplyFrame;

/// The `CLIENT SETNAME` command assigns a name to the current connection.
///
/// Reply:
/// - Simple string reply: OK if the connection name was successfully set.
pub fn set_name(name_ref: &mut Option<String>, new_name: String) -> ReplyFrame {
    *name_ref = Some(new_name);
    ReplyFrame::ok()
}
