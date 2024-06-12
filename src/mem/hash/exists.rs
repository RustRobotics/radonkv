// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::{Db, MemObject};

/// Returns if field is an existing field in the hash stored at key.
///
/// One of the following reply:
/// - Integer reply: 0 if the hash does not contain the field, or the key does not exist.
/// - Integer reply: 1 if the hash contains the field.
pub fn exists(db: &Db, key: &str, field: &str) -> ReplyFrame {
    match db.get(key) {
        Some(MemObject::Hash(old_hash)) => {
            if old_hash.contains_key(field) {
                ReplyFrame::one()
            } else {
                ReplyFrame::zero()
            }
        }
        Some(_) => ReplyFrame::wrong_type_err(),
        None => ReplyFrame::zero(),
    }
}

#[cfg(test)]
mod tests {
    use crate::cmd::reply_frame::ReplyFrame;
    use crate::mem::db::Db;
    use crate::mem::hash::exists::exists;
    use crate::mem::hash::set::set;

    #[test]
    fn test_exists() {
        let mut db = Db::new();
        let key = "myhash".to_owned();
        let reply = set(
            &mut db,
            key.clone(),
            vec![("field1".to_owned(), b"foo".to_vec())],
        );
        assert_eq!(reply, ReplyFrame::one());
        let reply = exists(&mut db, &key, "field1");
        assert_eq!(reply, ReplyFrame::one());
        let reply = exists(&mut db, &key, "field2");
        assert_eq!(reply, ReplyFrame::zero());
    }
}
