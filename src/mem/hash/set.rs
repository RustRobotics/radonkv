// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use std::collections::hash_map::Entry;

use crate::cmd::hash::ExtraValues;
use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::{Db, MemObject};
use crate::mem::hash::{append_to_hash, HashObject};

/// Sets the specified fields to their respective values in the hash stored at key.
///
/// This command overwrites the values of specified fields that exist in the hash.
/// If key doesn't exist, a new key holding a hash is created.
///
/// Reply:
/// - Integer reply: the number of fields that were added.
pub fn set(
    db: &mut Db,
    key: String,
    field: String,
    value: Vec<u8>,
    extra_values: ExtraValues,
) -> ReplyFrame {
    match db.entry(key) {
        Entry::Occupied(mut occupied) => match occupied.get_mut() {
            MemObject::Hash(old_hash) => {
                let count = append_to_hash(old_hash, field, value, extra_values);
                ReplyFrame::Usize(count)
            }
            _ => ReplyFrame::wrong_type_err(),
        },
        Entry::Vacant(vacant) => {
            let mut new_hash = HashObject::new();
            let count = append_to_hash(&mut new_hash, field, value, extra_values);
            vacant.insert(MemObject::Hash(new_hash));
            ReplyFrame::Usize(count)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cmd::reply_frame::ReplyFrame;
    use crate::mem::db::Db;
    use crate::mem::hash::get::get;
    use crate::mem::hash::get_all::get_all;
    use crate::mem::hash::set::set;

    #[test]
    fn test_set() {
        let mut db = Db::new();
        let key = "myhash".to_owned();
        let reply = set(
            &mut db,
            key.clone(),
            "field1".to_owned(),
            b"Hello".to_vec(),
            None,
        );
        assert_eq!(reply, ReplyFrame::Usize(1));

        let reply = get(&db, &key, "field");
        assert_eq!(reply, ReplyFrame::Bulk(b"Hello".to_vec()));

        let reply = set(
            &mut db,
            key.clone(),
            "field2".to_owned(),
            b"Hi".to_vec(),
            Some(vec![("field3".to_owned(), b"World".to_vec())]),
        );
        assert_eq!(reply, ReplyFrame::Usize(2));

        let reply = get(&db, &key, "field2");
        assert_eq!(reply, ReplyFrame::Bulk(b"Hi".to_vec()));
        let reply = get(&db, &key, "field3");
        assert_eq!(reply, ReplyFrame::Bulk(b"World".to_vec()));

        let reply = get_all(&db, &key);
        assert_eq!(
            reply,
            ReplyFrame::Array(vec![
                ReplyFrame::Bulk(b"field".to_vec()),
                ReplyFrame::Bulk(b"Hello".to_vec()),
                ReplyFrame::Bulk(b"field2".to_vec()),
                ReplyFrame::Bulk(b"Hi".to_vec()),
                ReplyFrame::Bulk(b"field3".to_vec()),
                ReplyFrame::Bulk(b"World".to_vec()),
            ])
        );
    }
}
