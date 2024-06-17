// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::reply_frame::ReplyFrame;

/// Quit the server.
/// If server receives one of the signals `SIGTERM` and `SIGINT`, the same shutdown sequence is performed.
///
/// Reply:
/// - On successful shutdown, nothing is returned because the server quits and the connection is closed.
///   On failure, an error is returned.
pub fn shutdown() -> ReplyFrame {
    todo!()
}
