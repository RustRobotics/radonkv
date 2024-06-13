// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::{Db, MemObject};

/// Returns if member is a member of the set stored at key.
///
/// One of the following reply:
/// - Integer reply: 0 if the element is not a member of the set, or when the key does not exist.
/// - Integer reply: 1 if the element is a member of the set.

pub fn is_member(db: &Db, key: &str, member: &[u8]) -> ReplyFrame {
    match db.get(key) {
        Some(MemObject::Set(old_set)) => {
            let present = old_set.contains(member);
            ReplyFrame::Usize(if present { 1 } else { 0 })
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
    use crate::mem::set::is_member::is_member;

    #[test]
    fn test_is_member() {
        let mut db = Db::new();
        let key = "myset".to_owned();
        let reply = add(&mut db, key.clone(), vec![b"one".to_vec()]);
        assert_eq!(reply, ReplyFrame::one());

        let reply = is_member(&db, &key, b"one");
        assert_eq!(reply, ReplyFrame::one());
        let reply = is_member(&db, &key, b"two");
        assert_eq!(reply, ReplyFrame::zero());
    }
}
