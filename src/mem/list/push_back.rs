// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use std::collections::hash_map::Entry;

use crate::cmd::list::ExtraValues;
use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::{Db, MemObject};
use crate::mem::list::ListObject;

/// Insert all the specified values at the tail of the list stored at key.
///
/// If key does not exist, it is created as empty list before performing the push operation.
/// When key holds a value that is not a list, an error is returned.
///
/// It is possible to push multiple elements using a single command call
/// just specifying multiple arguments at the end of the command.
/// Elements are inserted one after the other to the tail of the list, from the leftmost element
/// to the rightmost element.
///
/// So for instance the command `RPUSH mylist a b c` will result into a list containing
/// `a` as first element, `b` as second element and `c` as third element.
///
/// Reply:
/// - Integer reply: the length of the list after the push operation.
pub fn push_back(
    db: &mut Db,
    key: String,
    value: Vec<u8>,
    extra_values: ExtraValues,
) -> ReplyFrame {
    match db.entry(key) {
        Entry::Occupied(mut occupied) => match occupied.get_mut() {
            MemObject::List(old_list) => {
                old_list.push_back(value);
                if let Some(extra_values) = extra_values {
                    for extra_value in extra_values {
                        old_list.push_back(extra_value);
                    }
                }
                ReplyFrame::Usize(old_list.len())
            }
            _ => ReplyFrame::wrong_type_err(),
        },
        Entry::Vacant(vacant) => {
            // Keep order of items in values.
            let mut list = ListObject::new();
            list.push_back(value);
            if let Some(extra_values) = extra_values {
                for extra_value in extra_values {
                    list.push_back(extra_value);
                }
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
    use crate::mem::list::push_back::push_back;
    use crate::mem::list::range::range;

    #[test]
    fn test_push_back() {
        let mut db = Db::new();
        let key = "mylist".to_owned();
        let reply = push_back(&mut db, key.clone(), b"hello".to_vec(), None);
        assert_eq!(reply, ReplyFrame::Usize(1));

        let reply = push_back(&mut db, key.clone(), b"world".to_vec(), None);
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
