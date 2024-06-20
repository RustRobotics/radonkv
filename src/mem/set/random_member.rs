// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::{Db, MemObject};

/// When called with just the key argument, return a random element from the set value stored at key.
///
/// If the provided count argument is positive, return an array of distinct elements.
/// The array's length is either count or the set's cardinality (SCARD), whichever is lower.
///
/// If called with a negative count, the behavior changes and the command is allowed
/// to return the same element multiple times.
/// In this case, the number of returned elements is the absolute value of the specified count.
///
/// One of the following reply:
/// - Bulk string reply: without the additional count argument, the command returns
///   a randomly selected member, or a Nil reply when key doesn't exist.
/// - Array reply: when the optional count argument is passed, the command returns an array
///   of members, or an empty array when key doesn't exist.
pub fn random_member(db: &Db, key: &str, count: Option<isize>) -> ReplyFrame {
    match db.get(key) {
        Some(MemObject::Set(_old_set)) => {
            // TODO(Shaohua);
            todo!()
        }
        Some(_) => ReplyFrame::wrong_type_err(),
        None => {
            if count.is_some() {
                ReplyFrame::EmptyArray
            } else {
                ReplyFrame::Null
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cmd::reply_frame::ReplyFrame;
    use crate::mem::db::Db;
    use crate::mem::set::add::add;
    use crate::mem::set::random_member::random_member;

    #[test]
    fn test_random_member() {
        let mut db = Db::new();
        let key = "myset".to_owned();
        let reply = add(
            &mut db,
            key.clone(),
            vec![b"one".to_vec(), b"two".to_vec(), b"three".to_vec()],
        );
        assert_eq!(reply, ReplyFrame::Usize(3));
        let reply = random_member(&db, &key, None);
        assert!([
            ReplyFrame::Bulk(b"one".to_vec()),
            ReplyFrame::Bulk(b"two".to_vec()),
            ReplyFrame::Bulk(b"three".to_vec())
        ]
        .contains(&reply));
    }
}
