// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use std::collections::hash_map::Entry;
use std::mem;

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::{Db, MemObject};
use crate::mem::string::StrObject;

/// Sets the given keys to their respective values.
///
/// `MSET` replaces existing values with new values, just as regular `SET`.
/// See `MSETNX` if you don't want to overwrite existing values.
///
/// `MSET` is atomic, so all given keys are set at once.
/// It is not possible for clients to see that some of the keys were updated while others are unchanged.
///
/// Reply:
/// - Simple string reply: always OK because `MSET` can't fail.
pub fn multi_set(db: &mut Db, pairs: Vec<(String, Vec<u8>)>) -> ReplyFrame {
    for (key, value) in pairs {
        match db.entry(key) {
            Entry::Occupied(mut occupied) => match occupied.get_mut() {
                MemObject::Str(old_str) => {
                    // TODO(Shaohua): Replace with mem::swap() ?
                    old_str.vec = value;
                }
                other => {
                    let mut new_str = StrObject::from_bytes(value);
                    mem::swap(other, &mut new_str);
                }
            },
            Entry::Vacant(vacant) => {
                let new_str = StrObject::from_bytes(value);
                vacant.insert(new_str);
            }
        }
    }
    ReplyFrame::ok()
}

#[cfg(test)]
mod tests {
    use crate::cmd::reply_frame::ReplyFrame;
    use crate::mem::db::Db;
    use crate::mem::string::get::get;
    use crate::mem::string::multi_set::multi_set;

    #[test]
    fn test_multi_set() {
        let mut db = Db::new();
        let reply = multi_set(
            &mut db,
            vec![
                ("key1".to_owned(), b"Hello".to_vec()),
                ("key2".to_owned(), b"World".to_vec()),
            ],
        );
        assert_eq!(reply, ReplyFrame::ok());
        let reply = get(&db, "key1");
        assert_eq!(reply, ReplyFrame::Bulk(b"Hello".to_vec()));
        let reply = get(&db, "key2");
        assert_eq!(reply, ReplyFrame::Bulk(b"World".to_vec()));
    }
}
