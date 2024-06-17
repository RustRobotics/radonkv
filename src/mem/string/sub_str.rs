// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::{Db, MemObject};
use crate::mem::util::prune_range;

/// Returns the substring of the string value stored at key,
/// determined by the offsets start and end (both are inclusive).
///
/// Negative offsets can be used in order to provide an offset starting from the end of the string.
/// So -1 means the last character, -2 the penultimate and so forth.
///
/// The function handles out of range requests by limiting the resulting range to
/// the actual length of the string.
///
/// Reply:
/// - Bulk string reply: the substring of the string value stored at key,
///   determined by the offsets start and end (both are inclusive).
pub fn sub_str(db: &Db, key: &str, start: isize, end: isize) -> ReplyFrame {
    match db.get(key) {
        Some(MemObject::Str(old_str)) => {
            if let Some((start, end)) = prune_range(old_str.len(), start, end) {
                ReplyFrame::Bulk(old_str.vec[start..=end].to_vec())
            } else {
                ReplyFrame::EmptyBulk
            }
        }
        Some(_other) => ReplyFrame::wrong_type_err(),
        None => ReplyFrame::Null,
    }
}

#[cfg(test)]
mod tests {
    use crate::cmd::reply_frame::ReplyFrame;
    use crate::mem::db::Db;
    use crate::mem::string::set::set;
    use crate::mem::string::sub_str::sub_str;

    #[test]
    fn test_sub_str() {
        let mut db = Db::new();
        let key = "mykey".to_owned();
        let reply = set(&mut db, key.clone(), b"This is a string".to_vec());
        assert_eq!(reply, ReplyFrame::ok());
        let reply = sub_str(&db, &key, 0, 3);
        assert_eq!(reply, ReplyFrame::bulk(b"This".to_vec()));
        let reply = sub_str(&db, &key, -3, -1);
        assert_eq!(reply, ReplyFrame::bulk(b"ing".to_vec()));
        let reply = sub_str(&db, &key, 0, -1);
        assert_eq!(reply, ReplyFrame::bulk(b"This is a string".to_vec()));
        let reply = sub_str(&db, &key, 10, 100);
        assert_eq!(reply, ReplyFrame::bulk(b"string".to_vec()));
    }
}
