// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

#![allow(clippy::cast_possible_wrap)]

use std::collections::hash_map::Entry;

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::{Db, MemObject};
use crate::mem::string::StrObject;

/// If key already exists and is a string, this command appends the value at the end of the string.
/// If key does not exist it is created and set as an empty string,
/// so APPEND will be similar to SET in this special case.
///
/// Returns new length of string.
///
/// Reply:
/// - Integer reply: the length of the string after the append operation.
pub fn append(db: &mut Db, key: String, value: Vec<u8>) -> ReplyFrame {
    match db.entry(key) {
        Entry::Occupied(mut occupied) => match occupied.get_mut() {
            MemObject::Str(old_str) => {
                old_str.append(value);
                ReplyFrame::Usize(old_str.len())
            }
            _ => ReplyFrame::wrong_type_err(),
        },
        Entry::Vacant(vacant) => {
            let len = value.len();
            vacant.insert(StrObject::from_bytes(value));
            ReplyFrame::Usize(len)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cmd::reply_frame::ReplyFrame;
    use crate::mem::db::Db;
    use crate::mem::string::append::append;
    use crate::mem::string::get_range::get_range;

    #[test]
    fn test_get() {
        let mut db = Db::new();
        let key = "ts".to_owned();
        let reply = append(&mut db, key.clone(), b"0043".to_vec());
        assert_eq!(reply, ReplyFrame::Usize(4));
        let reply = append(&mut db, key.clone(), b"0035".to_vec());
        assert_eq!(reply, ReplyFrame::Usize(8));
        let reply = get_range(&db, &key, 0, 3);
        assert_eq!(reply, ReplyFrame::bulk(b"0043".to_vec()));
        let reply = get_range(&db, &key, 4, 7);
        assert_eq!(reply, ReplyFrame::bulk(b"0035".to_vec()));
    }
}
