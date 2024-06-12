// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::{Db, MemObject};

/// Returns the number of fields contained in the hash stored at key.
///
/// Reply:
/// - Integer reply: the number of fields in the hash, or 0 when the key does not exist.
pub fn len(db: &Db, key: &str) -> ReplyFrame {
    match db.get(key) {
        Some(MemObject::Hash(old_hash)) => ReplyFrame::Usize(old_hash.len()),
        Some(_) => ReplyFrame::wrong_type_err(),
        None => ReplyFrame::zero(),
    }
}

#[cfg(test)]
mod tests {
    use crate::cmd::reply_frame::ReplyFrame;
    use crate::mem::db::Db;
    use crate::mem::hash::len::len;
    use crate::mem::hash::set::set;

    #[test]
    fn test_len() {
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
        let reply = set(
            &mut db,
            key.clone(),
            "field2".to_owned(),
            b"World".to_vec(),
            None,
        );
        assert_eq!(reply, ReplyFrame::Usize(1));
        let reply = len(&db, &key);
        assert_eq!(reply, ReplyFrame::Usize(2));
    }
}
