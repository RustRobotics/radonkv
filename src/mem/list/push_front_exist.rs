// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use std::collections::hash_map::Entry;

use crate::cmd::list::ExtraValues;
use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::{Db, MemObject};

/// Inserts specified values at the head of the list stored at key,
/// only if key already exists and holds a list.
///
/// In contrary to `LPUSH`, no operation will be performed when key does not yet exist.
///
/// Reply:
// - Integer reply: the length of the list after the push operation.
pub fn push_front_exist(
    db: &mut Db,
    key: String,
    value: Vec<u8>,
    extra_values: ExtraValues,
) -> ReplyFrame {
    match db.entry(key) {
        Entry::Occupied(mut occupied) => match occupied.get_mut() {
            MemObject::Str(_) => ReplyFrame::wrong_type_err(),
            MemObject::List(old_list) => {
                old_list.push_front(value);
                if let Some(extra_values) = extra_values {
                    for extra_value in extra_values {
                        old_list.push_front(extra_value);
                    }
                }
                ReplyFrame::Usize(old_list.len())
            }
        },
        Entry::Vacant(_vacant) => ReplyFrame::zero(),
    }
}

#[cfg(test)]
mod tests {
    use crate::cmd::reply_frame::ReplyFrame;
    use crate::mem::db::Db;
    use crate::mem::list::push_front::push_front;
    use crate::mem::list::push_front_exist::push_front_exist;
    use crate::mem::list::range::range;

    #[test]
    fn test_push_front_exist() {
        let mut db = Db::new();
        let key = "mylist".to_owned();
        let reply = push_front(&mut db, key.clone(), b"World".to_vec(), None);
        assert_eq!(reply, ReplyFrame::Usize(1));

        let reply = push_front_exist(&mut db, key.clone(), b"Hello".to_vec(), None);
        assert_eq!(reply, ReplyFrame::Usize(2));
        let reply = push_front_exist(&mut db, "myotherlist".to_owned(), b"Hello".to_vec(), None);
        assert_eq!(reply, ReplyFrame::Usize(0));

        let reply = range(&mut db, &key, 0, -1);
        assert_eq!(
            reply,
            ReplyFrame::Array(vec![
                ReplyFrame::Bulk(b"Hello".to_vec()),
                ReplyFrame::Bulk(b"World".to_vec()),
            ])
        );

        let reply = range(&mut db, "myotherlist", 0, -1);
        assert_eq!(reply, ReplyFrame::EmptyArray);
    }
}
