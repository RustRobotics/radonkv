// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::{Db, MemObject};

/// Returns the set cardinality (number of elements) of the set stored at key.
///
/// Reply:
/// - Integer reply: the cardinality (number of elements) of the set, or 0 if the key does not exist.
pub fn len(db: &Db, key: &str) -> ReplyFrame {
    match db.get(key) {
        Some(MemObject::Set(old_set)) => {
            let len = old_set.len();
            ReplyFrame::Usize(len)
        }
        Some(_) => ReplyFrame::wrong_type_err(),
        None => ReplyFrame::zero(),
    }
}

#[cfg(test)]
mod tests {
    use crate::cmd::reply_frame::ReplyFrame;
    use crate::mem::db::Db;
    use crate::mem::set::add::add;
    use crate::mem::set::len::len;

    #[test]
    fn test_add() {
        let mut db = Db::new();
        let key = "myset".to_owned();
        let reply = add(&mut db, key.clone(), vec![b"Hello".to_vec()]);
        assert_eq!(reply, ReplyFrame::one());
        let reply = add(&mut db, key.clone(), vec![b"World".to_vec()]);
        assert_eq!(reply, ReplyFrame::one());

        let reply = len(&db, &key);
        assert_eq!(reply, ReplyFrame::Usize(2));
    }
}
