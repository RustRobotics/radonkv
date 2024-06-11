// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use std::collections::hash_map::Entry;

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::{Db, MemObject};

/// Removes and returns the first elements of the list stored at key.
///
/// By default, the command pops a single element from the beginning of the list.
///
/// When provided with the optional count argument, the reply will consist of up to count elements,
/// depending on the list's length.
pub fn pop_front(db: &mut Db, key: String, count: Option<usize>) -> ReplyFrame {
    match db.entry(key) {
        Entry::Occupied(mut occupied) => match occupied.get_mut() {
            MemObject::Str(_) => ReplyFrame::wrong_type_err(),
            MemObject::List(old_list) => {
                if let Some(count) = count {
                    let real_count: usize = count.min(old_list.len());
                    let mut array = Vec::new();
                    for _i in 0..real_count {
                        if let Some(item) = old_list.pop_front() {
                            array.push(ReplyFrame::Bulk(item));
                        } else {
                            break;
                        };
                    }
                    ReplyFrame::Array(array)
                } else {
                    // Returns the first front node.
                    if let Some(value) = old_list.pop_front() {
                        ReplyFrame::Bulk(value)
                    } else {
                        ReplyFrame::Null
                    }
                }
            }
        },
        Entry::Vacant(_) => ReplyFrame::Null,
    }
}

#[cfg(test)]
mod tests {
    use crate::cmd::reply_frame::ReplyFrame;
    use crate::mem::db::Db;
    use crate::mem::list::pop_front::pop_front;
    use crate::mem::list::push_back::push_back;
    use crate::mem::list::range::range;

    #[test]
    fn test_pop_front() {
        let mut db = Db::new();
        let key = "mylist".to_owned();
        let reply = push_back(
            &mut db,
            key.clone(),
            b"one".to_vec(),
            Some(vec![
                b"two".to_vec(),
                b"three".to_vec(),
                b"four".to_vec(),
                b"five".to_vec(),
            ]),
        );
        assert_eq!(reply, ReplyFrame::Usize(5));

        let reply = pop_front(&mut db, key.clone(), None);
        assert_eq!(reply, ReplyFrame::Bulk(b"one".to_vec()));
        let reply = pop_front(&mut db, key.clone(), Some(2));
        assert_eq!(
            reply,
            ReplyFrame::Array(vec![
                ReplyFrame::Bulk(b"two".to_vec()),
                ReplyFrame::Bulk(b"three".to_vec()),
            ])
        );
        let reply = range(&mut db, &key, 0, -1);
        assert_eq!(
            reply,
            ReplyFrame::Array(vec![
                ReplyFrame::Bulk(b"four".to_vec()),
                ReplyFrame::Bulk(b"five".to_vec()),
            ])
        );
    }
}
