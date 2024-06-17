// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::{Db, MemObject};
use crate::mem::string::StrObject;

/// Atomically sets key to value and returns the old value stored at key.
///
/// Returns an error when key exists but does not hold a string value.
/// Any previous time to live associated with the key is discarded on successful SET operation.
///
/// ## RESP2 Reply
///
/// One of the following:
/// - Bulk string reply: the old value stored at the key.
/// - Nil reply: if the key does not exist.
///
/// ## RESP3 Reply
///
/// One of the following:
/// - Bulk string reply: the old value stored at the key.
/// - Null reply: if the key does not exist.
pub fn get_set(db: &mut Db, key: String, value: Vec<u8>) -> ReplyFrame {
    match db.get(&key) {
        Some(MemObject::Str(old_value)) => {
            let frame = old_value.to_bulk();
            db.insert(key, StrObject::from_bytes(value));
            frame
        }
        Some(_other) => ReplyFrame::wrong_type_err(),
        None => {
            db.insert(key, StrObject::from_bytes(value));
            ReplyFrame::Null
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cmd::reply_frame::ReplyFrame;
    use crate::mem::db::Db;
    use crate::mem::string::get::get;
    use crate::mem::string::get_set::get_set;
    use crate::mem::string::set::set;

    #[test]
    fn test_get_set() {
        let mut db = Db::new();
        let key = "mykey".to_owned();
        let reply = set(&mut db, key.clone(), b"Hello".to_vec());
        assert_eq!(reply, ReplyFrame::ok());
        let reply = get_set(&mut db, key.clone(), b"World".to_vec());
        assert_eq!(reply, ReplyFrame::Bulk(b"Hello".to_vec()));
        let reply = get(&mut db, &key);
        assert_eq!(reply, ReplyFrame::Bulk(b"World".to_vec()));
    }
}
