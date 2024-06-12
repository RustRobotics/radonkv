// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::{Db, MemObject};

/// Removes and returns the last elements of the list stored at key.
///
/// By default, the command pops a single element from the end of the list.
/// When provided with the optional count argument, the reply will consist of up to count elements,
/// depending on the list's length.
///
/// One of the following reply:
/// - Nil reply: if the key does not exist.
/// - Bulk string reply: when called without the count argument, the value of the last element.
/// - Array reply: when called with the count argument, a list of popped elements.
pub fn pop_back(db: &mut Db, key: &str, count: Option<usize>) -> ReplyFrame {
    match db.get_mut(key) {
        Some(MemObject::List(old_list)) => {
            if let Some(count) = count {
                let real_count: usize = count.min(old_list.len());
                let mut array = Vec::new();
                for _i in 0..real_count {
                    if let Some(value) = old_list.pop_back() {
                        array.push(ReplyFrame::Bulk(value));
                    } else {
                        break;
                    };
                }
                ReplyFrame::Array(array)
            } else {
                if let Some(value) = old_list.pop_back() {
                    ReplyFrame::Bulk(value)
                } else {
                    ReplyFrame::Null
                }
            }
        }
        Some(_) => ReplyFrame::wrong_type_err(),
        None => ReplyFrame::Null,
    }
}

#[cfg(test)]
mod tests {
    use crate::cmd::reply_frame::ReplyFrame;
    use crate::mem::db::Db;
    use crate::mem::list::pop_back::pop_back;
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

        let reply = pop_back(&mut db, &key, None);
        assert_eq!(reply, ReplyFrame::Bulk(b"five".to_vec()));
        let reply = pop_back(&mut db, &key, Some(2));
        assert_eq!(
            reply,
            ReplyFrame::Array(vec![
                ReplyFrame::Bulk(b"four".to_vec()),
                ReplyFrame::Bulk(b"three".to_vec()),
            ])
        );
        let reply = range(&mut db, &key, 0, -1);
        assert_eq!(
            reply,
            ReplyFrame::Array(vec![
                ReplyFrame::Bulk(b"one".to_vec()),
                ReplyFrame::Bulk(b"two".to_vec()),
            ])
        );
    }
}
