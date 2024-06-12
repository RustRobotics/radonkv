// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::{Db, MemObject};

/// Removes the specified fields from the hash stored at key.
///
/// Specified fields that do not exist within this hash are ignored.
/// If key does not exist, it is treated as an empty hash and this command returns 0.
///
/// Reply:
/// - Integer reply: The number of fields that were removed from the hash,
///   excluding any specified but non-existing fields.
pub fn delete(db: &mut Db, key: &str, fields: &[String]) -> ReplyFrame {
    match db.get_mut(key) {
        Some(MemObject::Hash(old_hash)) => {
            let mut count = 0;
            for field in fields {
                if old_hash.remove(field).is_some() {
                    count += 1;
                }
            }

            ReplyFrame::Usize(count)
        }
        Some(_) => ReplyFrame::wrong_type_err(),
        None => ReplyFrame::zero(),
    }
}

#[cfg(test)]
mod tests {
    use crate::cmd::reply_frame::ReplyFrame;
    use crate::mem::db::Db;
    use crate::mem::hash::delete::delete;
    use crate::mem::hash::set::set;

    #[test]
    fn test_delete() {
        let mut db = Db::new();
        let key = "myhash".to_owned();
        let reply = set(
            &mut db,
            key.clone(),
            vec![("field1".to_owned(), b"foo".to_vec())],
        );
        assert_eq!(reply, ReplyFrame::one());
        let reply = delete(&mut db, &key, &["field1".to_owned()]);
        assert_eq!(reply, ReplyFrame::one());
        let reply = delete(&mut db, &key, &["field2".to_owned()]);
        assert_eq!(reply, ReplyFrame::zero());
    }
}
