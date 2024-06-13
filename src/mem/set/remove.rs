// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::{Db, MemObject};

/// Remove the specified members from the set stored at key.
///
/// Specified members that are not a member of this set are ignored. If key does not exist,
/// it is treated as an empty set and this command returns 0.
///
/// An error is returned when the value stored at key is not a set.
///
/// Reply:
/// - Integer reply: the number of members that were removed from the set,
///   not including non-existing members.
pub fn remove(db: &mut Db, key: &str, members: &[Vec<u8>]) -> ReplyFrame {
    match db.get_mut(key) {
        Some(MemObject::Set(old_set)) => {
            let old_len = old_set.len();
            for member in members {
                old_set.remove(member);
            }
            ReplyFrame::Usize(old_len - old_set.len())
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
    use crate::mem::set::members::members;
    use crate::mem::set::remove::remove;

    #[test]
    fn test_remove() {
        let mut db = Db::new();
        let key = "myset".to_owned();
        let reply = add(
            &mut db,
            key.clone(),
            vec![b"one".to_vec(), b"two".to_vec(), b"three".to_vec()],
        );
        assert_eq!(reply, ReplyFrame::Usize(3));
        let reply = remove(&mut db, &key, &[b"one".to_vec()]);
        assert_eq!(reply, ReplyFrame::one());
        let reply = remove(&mut db, &key, &[b"four".to_vec()]);
        assert_eq!(reply, ReplyFrame::zero());
        let reply = members(&db, &key);
        // TODO(Shaohua): Sort members based on insertion order.
        assert_eq!(
            reply,
            ReplyFrame::Array(vec![
                ReplyFrame::Bulk(b"three".to_vec()),
                ReplyFrame::Bulk(b"two".to_vec()),
            ])
        );
    }
}
