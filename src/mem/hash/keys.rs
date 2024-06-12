// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::{Db, MemObject};

/// Returns all field names in the hash stored at key.
///
/// Reply:
/// - Array reply: a list of fields in the hash, or an empty list when the key does not exist.
pub fn keys(db: &Db, key: &str) -> ReplyFrame {
    match db.get(key) {
        Some(MemObject::Hash(old_hash)) => {
            let mut keys: Vec<&String> = old_hash.keys().collect();
            keys.sort_unstable();
            let array: Vec<ReplyFrame> = keys
                .into_iter()
                .map(|key| ReplyFrame::Bulk(key.as_bytes().to_vec()))
                .collect();
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
    use crate::mem::hash::keys::keys;
    use crate::mem::hash::set::set;

    #[test]
    fn test_keys() {
        let mut db = Db::new();
        let key = "myhash".to_owned();
        let reply = set(
            &mut db,
            key.clone(),
            vec![
                ("field1".to_owned(), b"Hello".to_vec()),
                ("field2".to_owned(), b"World".to_vec()),
            ],
        );
        assert_eq!(reply, ReplyFrame::Usize(2));
        let reply = keys(&db, &key);
        assert_eq!(
            reply,
            ReplyFrame::Array(vec![
                ReplyFrame::Bulk(b"field1".to_vec()),
                ReplyFrame::Bulk(b"field2".to_vec()),
            ])
        );
    }
}
