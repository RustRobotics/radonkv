// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::{Db, MemObject};

/// Get the value of key and delete the key.
///
/// This command is similar to GET, except for the fact that it also deletes the key
/// on success (if and only if the key's value type is a string).
///
/// ## RESP2 Reply
///
/// One of the following:
/// - Bulk string reply: the value of the key.
/// - Nil reply: if the key does not exist or if the key's value type is not a string.
///
/// ## RESP3 Reply
///
/// One of the following:
/// - Bulk string reply: the value of the key.
/// - Null reply: if the key does not exist or if the key's value type is not a string.
pub fn get_del(db: &mut Db, key: &String) -> ReplyFrame {
    match db.get(key) {
        Some(MemObject::Str(value)) => {
            let frame = value.to_bulk();
            db.remove(key);
            frame
        }
        Some(_other) => ReplyFrame::wrong_type_err(),
        None => ReplyFrame::Null,
    }
}

#[cfg(test)]
mod tests {
    use crate::cmd::reply_frame::ReplyFrame;
    use crate::mem::db::Db;
    use crate::mem::string::get::get;
    use crate::mem::string::get_del::get_del;
    use crate::mem::string::set::set;

    #[test]
    fn test_get_del() {
        let mut db = Db::new();
        let key = "mykey".to_owned();
        let reply = set(&mut db, key.clone(), b"Hello".to_vec());
        assert_eq!(reply, ReplyFrame::ok());
        let reply = get_del(&mut db, &key);
        assert_eq!(reply, ReplyFrame::Bulk(b"Hello".to_vec()));
        let reply = get(&db, &key);
        assert_eq!(reply, ReplyFrame::Null);
    }
}
