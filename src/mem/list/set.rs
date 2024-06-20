// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::{Db, MemObject};
use crate::mem::util::prune_index;

/// Sets the list element at index to element.
///
/// The index is zero-based, so 0 means the first element, 1 the second element and so on.
/// Negative indices can be used to designate elements starting at the tail of the list.
/// Here, -1 means the last element, -2 means the penultimate and so forth.
///
/// When the value at key is not a list, an error is returned.
///
/// An error is returned for out of range indexes.
///
/// Reply:
/// - Simple string reply: OK.
pub fn set(db: &mut Db, key: &str, index: isize, value: Vec<u8>) -> ReplyFrame {
    match db.get_mut(key) {
        Some(MemObject::List(old_list)) => {
            prune_index(old_list.len(), index).map_or_else(ReplyFrame::out_of_range_err, |index| {
                // TODO(Shaohua): Simplify
                for (i, old_value) in old_list.iter_mut().enumerate() {
                    if i == index {
                        // TODO(Shaohua): Replace with mem::swap()
                        *old_value = value;
                        break;
                    }
                }
                ReplyFrame::ok()
            })
        }
        Some(_other) => ReplyFrame::wrong_type_err(),
        None => ReplyFrame::no_such_key(),
    }
}

#[cfg(test)]
mod tests {
    use crate::cmd::reply_frame::ReplyFrame;
    use crate::mem::db::Db;
    use crate::mem::list::push_back::push_back;
    use crate::mem::list::range::range;
    use crate::mem::list::set::set;

    #[test]
    fn test_set() {
        let mut db = Db::new();
        let key = "mylist".to_owned();
        let reply = push_back(&mut db, key.clone(), vec![b"one".to_vec()]);
        assert_eq!(reply, ReplyFrame::Usize(1));
        let reply = push_back(&mut db, key.clone(), vec![b"two".to_vec()]);
        assert_eq!(reply, ReplyFrame::Usize(2));
        let reply = push_back(&mut db, key.clone(), vec![b"three".to_vec()]);
        assert_eq!(reply, ReplyFrame::Usize(3));
        let reply = set(&mut db, &key, 0, b"four".to_vec());
        assert_eq!(reply, ReplyFrame::ok());
        let reply = set(&mut db, &key, -2, b"five".to_vec());
        assert_eq!(reply, ReplyFrame::ok());
        let reply = range(&db, &key, 0, -1);
        assert_eq!(
            reply,
            ReplyFrame::Array(vec![
                ReplyFrame::Bulk(b"four".to_vec()),
                ReplyFrame::Bulk(b"five".to_vec()),
                ReplyFrame::Bulk(b"three".to_vec()),
            ])
        );
    }
}
