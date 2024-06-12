// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::{Db, MemObject};

/// Returns the value associated with field in the hash stored at key.
///
/// Reply:
/// - Bulk string reply: The value associated with the field.
/// - Null reply: If the field is not present in the hash or key does not exist.
pub fn get(db: &Db, key: &str, field: &str) -> ReplyFrame {
    match db.get(key) {
        Some(MemObject::Hash(old_hash)) => old_hash
            .get(field)
            .cloned()
            .map_or_else(ReplyFrame::null, ReplyFrame::bulk),
        Some(_) => ReplyFrame::wrong_type_err(),
        None => ReplyFrame::Null,
    }
}

#[cfg(test)]
mod tests {
    use crate::cmd::reply_frame::ReplyFrame;
    use crate::mem::db::Db;
    use crate::mem::hash::get::get;
    use crate::mem::hash::set::set;

    #[test]
    fn test_get() {
        let mut db = Db::new();
        let key = "myhash".to_owned();
        let reply = set(
            &mut db,
            key.clone(),
            vec![("field1".to_owned(), b"foo".to_vec())],
        );
        assert_eq!(reply, ReplyFrame::Usize(1));

        let reply = get(&db, &key, "field1");
        assert_eq!(reply, ReplyFrame::Bulk(b"foo".to_vec()));
        let reply = get(&db, &key, "field2");
        assert_eq!(reply, ReplyFrame::Null);
    }
}
