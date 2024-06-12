// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use std::collections::hash_map::Entry;

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::{Db, MemObject};
use crate::mem::list::ListObject;

/// Insert all the specified values at the head of the list stored at key.
///
/// If key does not exist, it is created as empty list before performing the push operations.
/// When key holds a value that is not a list, an error is returned.
///
/// It is possible to push multiple elements using a single command call
/// just specifying multiple arguments at the end of the command.
/// Elements are inserted one after the other to the head of the list,
/// from the leftmost element to the rightmost element.
/// So for instance the command `LPUSH mylist a b c` will result into a list containing
/// `c` as first element, `b` as second element and `a` as third element.
///
/// Reply:
/// - Integer reply: the length of the list after the push operation.
pub fn push_front(db: &mut Db, key: String, values: Vec<Vec<u8>>) -> ReplyFrame {
    match db.entry(key) {
        Entry::Occupied(mut occupied) => match occupied.get_mut() {
            MemObject::List(old_list) => {
                for value in values {
                    old_list.push_front(value);
                }
                ReplyFrame::Usize(old_list.len())
            }
            _ => ReplyFrame::wrong_type_err(),
        },
        Entry::Vacant(vacant) => {
            // NOTE(Shaohua): Reverse order of items in values.
            let mut list = ListObject::new();
            for value in values {
                list.push_front(value);
            }
            let len = list.len();
            vacant.insert(MemObject::List(list));
            ReplyFrame::Usize(len)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cmd::reply_frame::ReplyFrame;
    use crate::mem::db::Db;
    use crate::mem::list::push_front::push_front;
    use crate::mem::list::range::range;

    #[test]
    fn test_push_front() {
        let mut db = Db::new();
        let key = "mylist".to_owned();
        let reply = push_front(&mut db, key.clone(), vec![b"world".to_vec()]);
        assert_eq!(reply, ReplyFrame::Usize(1));

        let reply = push_front(&mut db, key.clone(), vec![b"hello".to_vec()]);
        assert_eq!(reply, ReplyFrame::Usize(2));
        let reply = range(&mut db, &key, 0, -1);
        assert_eq!(
            reply,
            ReplyFrame::Array(vec![
                ReplyFrame::Bulk(b"hello".to_vec()),
                ReplyFrame::Bulk(b"world".to_vec()),
            ])
        );
    }
}
