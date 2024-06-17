// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::{Db, MemObject};

/// Returns the length of the string value stored at key.
///
/// An error is returned when key holds a non-string value.
///
/// Reply:
/// - Integer reply: the length of the string stored at key, or 0 when the key does not exist.
#[allow(clippy::cast_possible_wrap)]
pub fn len(db: &Db, key: &str) -> ReplyFrame {
    match db.get(key) {
        Some(MemObject::Str(value)) => ReplyFrame::Usize(value.len()),
        Some(_other) => ReplyFrame::wrong_type_err(),
        None => ReplyFrame::Usize(0),
    }
}

#[cfg(test)]
mod tests {
    use crate::cmd::reply_frame::ReplyFrame;
    use crate::mem::db::Db;
    use crate::mem::string::len::len;
    use crate::mem::string::set::set;

    #[test]
    fn test_len() {
        let mut db = Db::new();
        let key = "mykey".to_owned();
        let reply = set(&mut db, key.clone(), b"Hello world".to_vec());
        assert_eq!(reply, ReplyFrame::ok());
        let reply = len(&db, &key);
        assert_eq!(reply, ReplyFrame::Usize(11));
        let reply = len(&db, "nonexisting");
        assert_eq!(reply, ReplyFrame::zero());
    }
}
