// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use std::collections::hash_map::Entry;
use std::mem;

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::{Db, MemObject};
use crate::mem::string::StrObject;

/// Set key to hold the string value.
///
/// If key already holds a value, it is overwritten, regardless of its type.
/// Any previous time to live associated with the key is discarded on successful `SET` operation.
///
/// ## RESP2 Reply
///
/// Any of the following:
/// - Nil reply: GET not given: Operation was aborted (conflict with one of the XX/NX options).
/// - Simple string reply: OK. GET not given: The key was set.
/// - Nil reply: GET given: The key didn't exist before the SET.
/// - Bulk string reply: GET given: The previous value of the key.
///
/// ## RESP3 Reply
///
/// Any of the following:
/// - Null reply: GET not given: Operation was aborted (conflict with one of the XX/NX options).
/// - Simple string reply: OK. GET not given: The key was set.
/// - Null reply: GET given: The key didn't exist before the SET.
/// - Bulk string reply: GET given: The previous value of the key.
pub fn set(db: &mut Db, key: String, mut value: Vec<u8>) -> ReplyFrame {
    match db.entry(key) {
        Entry::Occupied(mut occupied) => match occupied.get_mut() {
            MemObject::Str(old_str) => {
                mem::swap(&mut old_str.vec, &mut value);
                ReplyFrame::ok()
            }
            old_obj => {
                let mut new_str = StrObject::from_bytes(value);
                mem::swap(old_obj, &mut new_str);
                ReplyFrame::ok()
            }
        },
        Entry::Vacant(vacant) => {
            vacant.insert(StrObject::from_bytes(value));
            ReplyFrame::ok()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cmd::reply_frame::ReplyFrame;
    use crate::mem::db::Db;
    use crate::mem::string::get::get;
    use crate::mem::string::set::set;

    #[test]
    fn test_set() {
        let mut db = Db::new();
        let key = "mykey".to_owned();
        let reply = set(&mut db, key.clone(), b"Hello".to_vec());
        assert_eq!(reply, ReplyFrame::ok());
        let reply = get(&db, &key);
        assert_eq!(reply, ReplyFrame::Bulk(b"Hello".to_vec()));
    }
}
