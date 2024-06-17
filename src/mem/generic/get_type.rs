// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::{Db, MemObject};

/// Returns the string representation of the type of the value stored at key.
///
/// The different types that can be returned are:
/// - string
/// - list
/// - set
/// - zset
/// - hash
/// - stream
///
/// Reply:
/// - Simple string reply: the type of key, or none when key doesn't exist.
pub fn get_type(db: &Db, key: &str) -> ReplyFrame {
    let obj_type = match db.get(key) {
        // Core objects
        Some(MemObject::Str(_)) => "string",
        Some(MemObject::List(_)) => "list",
        Some(MemObject::Hash(_)) => "hash",
        Some(MemObject::Set(_)) => "set",
        // TODO(Shaohua): Returns "string" instead of "hyper"
        Some(MemObject::Hyper(_)) => "hyper",

        // Stack objects
        Some(MemObject::BloomFilter(_)) => "bloom",

        None => "none",
    };
    ReplyFrame::ConstSimple(obj_type)
}

#[cfg(test)]
mod tests {
    use crate::cmd::reply_frame::ReplyFrame;
    use crate::mem::db::Db;
    use crate::mem::generic::get_type::get_type;
    use crate::mem::list::push_front::push_front;
    use crate::mem::set::add::add;
    use crate::mem::string::set::set;

    #[test]
    fn test_get_type() {
        let mut db = Db::new();
        let key1 = "key1".to_owned();
        let reply = set(&mut db, key1.clone(), b"value".to_vec());
        assert_eq!(reply, ReplyFrame::ok());
        let key2 = "key2".to_owned();
        let reply = push_front(&mut db, key2.clone(), vec![b"value".to_vec()]);
        assert_eq!(reply, ReplyFrame::one());
        let key3 = "key3".to_owned();
        let reply = add(&mut db, key3.clone(), vec![b"value".to_vec()]);
        assert_eq!(reply, ReplyFrame::one());
        let reply = get_type(&db, &key1);
        assert_eq!(reply, ReplyFrame::ConstSimple("string"));
        let reply = get_type(&db, &key2);
        assert_eq!(reply, ReplyFrame::ConstSimple("list"));
        let reply = get_type(&db, &key3);
        assert_eq!(reply, ReplyFrame::ConstSimple("set"));
    }
}
