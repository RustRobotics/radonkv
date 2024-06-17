// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::Db;
use crate::mem::string::sub_str::sub_str;

/// Returns the substring of the string value stored at key, determined by the offsets
/// start and end (both are inclusive).
///
/// Negative offsets can be used in order to provide an offset starting from the end of the string.
/// So -1 means the last character, -2 the penultimate and so forth.
///
/// The function handles out of range requests by limiting the resulting range
/// to the actual length of the string.
///
/// Reply:
/// - Bulk string reply: The substring of the string value stored at key,
///   determined by the offsets start and end (both are inclusive).
#[must_use]
#[inline]
pub fn get_range(db: &Db, key: &str, start: isize, end: isize) -> ReplyFrame {
    sub_str(db, key, start, end)
}

#[cfg(test)]
mod tests {
    use crate::cmd::reply_frame::ReplyFrame;
    use crate::mem::db::Db;
    use crate::mem::string::get_range::get_range;
    use crate::mem::string::set::set;

    #[test]
    fn test_get_range() {
        let mut db = Db::new();
        let key = "mykey".to_owned();
        let reply = set(&mut db, key.clone(), b"This is a string".to_vec());
        assert_eq!(reply, ReplyFrame::ok());
        let reply = get_range(&db, &key, 0, 3);
        assert_eq!(reply, ReplyFrame::bulk(b"This".to_vec()));
        let reply = get_range(&db, &key, -3, -1);
        assert_eq!(reply, ReplyFrame::bulk(b"ing".to_vec()));
        let reply = get_range(&db, &key, 0, -1);
        assert_eq!(reply, ReplyFrame::bulk(b"This is a string".to_vec()));
        let reply = get_range(&db, &key, 10, 100);
        assert_eq!(reply, ReplyFrame::bulk(b"string".to_vec()));
    }
}
