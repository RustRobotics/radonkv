// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::reply_frame::ReplyFrame;

/// Returns message.
///
/// Reply:
/// - Bulk string reply: the given string.
pub fn echo(message: String) -> ReplyFrame {
    ReplyFrame::Bulk(message.into_bytes())
}

#[cfg(test)]
mod tests {
    use crate::cmd::reply_frame::ReplyFrame;
    use crate::session::commands::echo::echo;

    #[test]
    fn test_echo() {
        let reply = echo("hello world".to_owned());
        assert_eq!(reply, ReplyFrame::bulk(b"hello world".to_vec()));
    }
}
