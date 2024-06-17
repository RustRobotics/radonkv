// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use std::time::SystemTime;

use crate::cmd::reply_frame::ReplyFrame;

/// The TIME command returns the current server time as a two items lists:
/// a Unix timestamp and the amount of microseconds already elapsed in the current second.
///
/// Basically the interface is very similar to the one of the `gettimeofday` system call.
///
/// Reply:
/// - Array reply: specifically, a two-element array consisting of the Unix timestamp in seconds
///   and the microseconds' count.
pub fn time() -> ReplyFrame {
    let now = SystemTime::now();
    match now.duration_since(SystemTime::UNIX_EPOCH) {
        Ok(duration) => {
            let secs = duration.as_secs();
            let micros = duration.subsec_micros();
            let vec = vec![ReplyFrame::I64(secs as i64), ReplyFrame::I64(micros as i64)];
            ReplyFrame::Array(vec)
        }
        Err(err) => {
            log::warn!("[server] failed to get system time, err: {err:?}");
            ReplyFrame::Null
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cmd::reply_frame::ReplyFrame;
    use crate::server::commands::time;

    #[test]
    fn test_time() {
        let reply = time();
        match reply {
            ReplyFrame::Array(vec) => {
                assert_eq!(vec.len(), 2);
            }
            _ => unreachable!(),
        }
    }
}
