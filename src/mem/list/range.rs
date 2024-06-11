// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::{Db, MemObject};
use crate::mem::list::range_to_reply_frame;

/// Returns the specified elements of the list stored at key.
///
/// The offsets start and stop are zero-based indexes, with 0 being the first element of the list
/// (the head of the list), 1 being the next element and so on.
//
// These offsets can also be negative numbers indicating offsets starting at the end of the list.
// For example, -1 is the last element of the list, -2 the penultimate, and so on.
pub fn range(db: &Db, key: &str, start: isize, end: isize) -> ReplyFrame {
    match db.get(key) {
        Some(MemObject::List(list)) => range_to_reply_frame(list, start, end),
        Some(_other) => ReplyFrame::wrong_type_err(),
        None => ReplyFrame::EmptyArray,
    }
}

#[cfg(test)]
mod tests {
    use crate::cmd::reply_frame::ReplyFrame;
    use crate::mem::db::Db;
    use crate::mem::list::push_back::push_back;
    use crate::mem::list::range::range;

    #[test]
    fn test_range() {
        let mut db = Db::new();
        let key = "mylist".to_owned();
        let reply = push_back(&mut db, key.clone(), b"one".to_vec(), None);
        assert_eq!(reply, ReplyFrame::Usize(1));
        let reply = push_back(&mut db, key.clone(), b"two".to_vec(), None);
        assert_eq!(reply, ReplyFrame::Usize(2));
        let reply = push_back(&mut db, key.clone(), b"three".to_vec(), None);
        assert_eq!(reply, ReplyFrame::Usize(3));

        let reply = range(&db, &key, 0, 0);
        assert_eq!(
            reply,
            ReplyFrame::Array(vec![ReplyFrame::Bulk(b"one".to_vec())])
        );
        let reply = range(&db, &key, -3, 2);
        assert_eq!(
            reply,
            ReplyFrame::Array(vec![
                ReplyFrame::Bulk(b"one".to_vec()),
                ReplyFrame::Bulk(b"two".to_vec()),
                ReplyFrame::Bulk(b"three".to_vec()),
            ])
        );

        let reply = range(&db, &key, -100, 100);
        assert_eq!(
            reply,
            ReplyFrame::Array(vec![
                ReplyFrame::Bulk(b"one".to_vec()),
                ReplyFrame::Bulk(b"two".to_vec()),
                ReplyFrame::Bulk(b"three".to_vec()),
            ])
        );

        let reply = range(&db, &key, 5, 10);
        assert_eq!(reply, ReplyFrame::EmptyArray);
    }
}
