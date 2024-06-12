// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::{Db, MemObject};

/// Returns the string length of the value associated with field in the hash stored at key.
///
/// If the key or the field do not exist, 0 is returned.
///
/// Reply:
/// - Integer reply: the string length of the value associated with the field,
///   or zero when the field isn't present in the hash or the key doesn't exist at all.
pub fn str_len(db: &Db, key: &str, field: &str) -> ReplyFrame {
    match db.get(key) {
        Some(MemObject::Hash(old_hash)) => {
            if let Some(value) = old_hash.get(field) {
                ReplyFrame::Usize(value.len())
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
    use crate::mem::hash::set::set;
    use crate::mem::hash::str_len::str_len;

    #[test]
    fn test_str_len() {
        let mut db = Db::new();
        let key = "myhash".to_owned();
        let reply = set(
            &mut db,
            key.clone(),
            "f1".to_owned(),
            b"HelloWorld".to_vec(),
            Some(vec![
                ("f2".to_owned(), b"99".to_vec()),
                ("f3".to_owned(), b"-256".to_vec()),
            ]),
        );
        assert_eq!(reply, ReplyFrame::Usize(3));

        let reply = str_len(&db, &key, "f1");
        assert_eq!(reply, ReplyFrame::Usize(10));
        let reply = str_len(&db, &key, "f2");
        assert_eq!(reply, ReplyFrame::Usize(2));
        let reply = str_len(&db, &key, "f3");
        assert_eq!(reply, ReplyFrame::Usize(4));
    }
}
