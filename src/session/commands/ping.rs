// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::reply_frame::ReplyFrame;

/// Returns PONG if no argument is provided, otherwise return a copy of the argument as a bulk.
///
/// This command is useful for:
/// - Testing whether a connection is still alive.
/// - Verifying the server's ability to serve data - an error is returned when this isn't the case (e.g., during load from persistence or accessing a stale replica).
/// - Measuring latency.
///
/// If the client is subscribed to a channel or a pattern, it will instead return a multi-bulk
/// with a "pong" in the first position and an empty bulk in the second position,
/// unless an argument is provided in which case it returns a copy of the argument.
///
/// Any of the following reply:
/// - Simple string reply: PONG when no argument is provided.
/// - Bulk string reply: the provided argument.
pub fn ping(message: Option<String>) -> ReplyFrame {
    message.map_or_else(ReplyFrame::pong, |message| {
        ReplyFrame::Bulk(message.into_bytes())
    })
}

#[cfg(test)]
mod tests {
    use crate::cmd::reply_frame::ReplyFrame;
    use crate::session::commands::ping::ping;

    #[test]
    fn test_ping() {
        let reply = ping(None);
        assert_eq!(reply, ReplyFrame::pong());
        let reply = ping(Some("hello world".to_owned()));
        assert_eq!(reply, ReplyFrame::bulk(b"hello world".to_vec()));
    }
}
