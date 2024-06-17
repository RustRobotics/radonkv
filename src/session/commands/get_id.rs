// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::reply_frame::ReplyFrame;
use crate::listener::types::SessionId;

/// The command just returns the ID of the current connection.
///
/// Reply:
/// - Integer reply: the ID of the client.
#[must_use]
#[inline]
pub const fn get_id(id: SessionId) -> ReplyFrame {
    ReplyFrame::I64(id)
}
