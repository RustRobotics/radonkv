// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use std::collections::hash_map::Entry;

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::{Db, MemObject};
use crate::mem::set::SetObject;

/// Add the specified members to the set stored at key.
///
/// Specified members that are already a member of this set are ignored.
/// If key does not exist, a new set is created before adding the specified members.
///
/// An error is returned when the value stored at key is not a set.
///
/// Reply:
/// - Integer reply: the number of elements that were added to the set,
///   not including all the elements already present in the set.
pub fn add(db: &mut Db, key: String, members: Vec<Vec<u8>>) -> ReplyFrame {
    match db.entry(key) {
        Entry::Occupied(mut occupied) => match occupied.get_mut() {
            MemObject::Set(old_set) => {
                let old_len = old_set.len();
                for member in members {
                    old_set.insert(member);
                }
                ReplyFrame::Usize(old_set.len() - old_len)
            }
            _ => ReplyFrame::wrong_type_err(),
        },
        Entry::Vacant(vacant) => {
            let new_set: SetObject = members.into_iter().collect();
            let len = new_set.len();
            vacant.insert(MemObject::Set(new_set));
            ReplyFrame::Usize(len)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cmd::reply_frame::ReplyFrame;
    use crate::mem::db::Db;
    use crate::mem::set::add::add;
    use crate::mem::set::members::members;

    #[test]
    fn test_add() {
        let mut db = Db::new();
        let key = "myset".to_owned();
        let reply = add(&mut db, key.clone(), vec![b"Hello".to_vec()]);
        assert_eq!(reply, ReplyFrame::one());
        let reply = add(&mut db, key.clone(), vec![b"World".to_vec()]);
        assert_eq!(reply, ReplyFrame::one());
        let reply = add(&mut db, key.clone(), vec![b"World".to_vec()]);
        assert_eq!(reply, ReplyFrame::zero());
        let reply = members(&db, &key);
        assert_eq!(
            reply,
            ReplyFrame::Array(vec![
                ReplyFrame::Bulk(b"Hello".to_vec()),
                ReplyFrame::Bulk(b"World".to_vec()),
            ])
        )
    }
}
