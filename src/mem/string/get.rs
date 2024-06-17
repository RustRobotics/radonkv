// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::{Db, MemObject};

/// Get the value of key.
///
/// If the key does not exist the special value nil is returned.
/// An error is returned if the value stored at key is not a string,
/// because GET only handles string values.
///
/// ## RESP2 Reply
///
/// One of the following:
/// - Bulk string reply: the value of the key.
/// - Nil reply: if the key does not exist.
///
/// ## RESP3 Reply
///
/// One of the following:
/// - Bulk string reply: the value of the key.
/// - Null reply: key does not exist.
pub fn get(db: &Db, key: &str) -> ReplyFrame {
    match db.get(key) {
        Some(MemObject::Str(value)) => value.to_bulk(),
        Some(_other) => ReplyFrame::wrong_type_err(),
        None => ReplyFrame::Null,
    }
}

#[cfg(test)]
mod tests {
    use crate::cmd::reply_frame::ReplyFrame;
    use crate::mem::db::Db;
    use crate::mem::string::get::get;
    use crate::mem::string::set::set;

    #[test]
    fn test_get() {
        let mut db = Db::new();
        let reply = get(&db, "nonexisting");
        assert_eq!(reply, ReplyFrame::Null);
        let key = "mykey".to_owned();
        let reply = set(&mut db, key.clone(), b"Hello".to_vec());
        assert_eq!(reply, ReplyFrame::ok());
        let reply = get(&db, &key);
        assert_eq!(reply, ReplyFrame::bulk(b"Hello".to_vec()));
    }
}
