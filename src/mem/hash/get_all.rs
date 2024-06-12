// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::{Db, MemObject};

/// Returns all fields and values of the hash stored at key.
///
/// In the returned value, every field name is followed by its value,
/// so the length of the reply is twice the size of the hash.
///
/// RESP2 Reply:
/// - Array reply: a list of fields and their values stored in the hash,
///   or an empty list when key does not exist.
pub fn get_all(db: &Db, key: &str) -> ReplyFrame {
    match db.get(key) {
        Some(MemObject::Hash(old_hash)) => {
            let mut keys: Vec<_> = old_hash.keys().collect();
            keys.sort_unstable();

            let mut array = Vec::new();
            for field in keys {
                if let Some(value) = old_hash.get(field) {
                    array.push(ReplyFrame::Bulk(field.as_bytes().to_vec()));
                    array.push(ReplyFrame::Bulk(value.clone()));
                }
            }
            ReplyFrame::Array(array)
        }
        Some(_) => ReplyFrame::wrong_type_err(),
        None => ReplyFrame::EmptyArray,
    }
}

#[cfg(test)]
mod tests {
    use crate::cmd::reply_frame::ReplyFrame;
    use crate::mem::db::Db;
    use crate::mem::hash::get_all::get_all;
    use crate::mem::hash::set::set;

    #[test]
    fn test_get_all() {
        let mut db = Db::new();
        let key = "myhash".to_owned();
        let reply = set(
            &mut db,
            key.clone(),
            vec![("field1".to_owned(), b"Hello".to_vec())],
        );
        assert_eq!(reply, ReplyFrame::Usize(1));
        let reply = set(
            &mut db,
            key.clone(),
            vec![("field2".to_owned(), b"World".to_vec())],
        );
        assert_eq!(reply, ReplyFrame::Usize(1));
        let reply = get_all(&db, &key);
        assert_eq!(
            reply,
            ReplyFrame::Array(vec![
                ReplyFrame::Bulk(b"field1".to_vec()),
                ReplyFrame::Bulk(b"Hello".to_vec()),
                ReplyFrame::Bulk(b"field2".to_vec()),
                ReplyFrame::Bulk(b"World".to_vec()),
            ])
        );
    }
}
