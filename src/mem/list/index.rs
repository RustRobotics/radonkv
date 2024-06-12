// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::{Db, MemObject};
use crate::mem::util::prune_index;

/// Returns the element at `index` in the list stored at key.
///
/// The index is zero-based, so 0 means the first element, 1 the second element and so on.
/// Negative indices can be used to designate elements starting at the tail of the list.
/// Here, -1 means the last element, -2 means the penultimate and so forth.
///
/// When the value at key is not a list, an error is returned.
///
/// One of the following reply:
/// - Null reply: when index is out of range.
/// - Bulk string reply: the requested element.
pub fn index(db: &Db, key: &str, index: isize) -> ReplyFrame {
    match db.get(key) {
        Some(MemObject::List(list)) => {
            if let Some(real_index) = prune_index(list.len(), index) {
                // TODO(Shaohua): Use iter().skip().take()
                for (index, item) in list.iter().enumerate() {
                    if index == real_index {
                        return ReplyFrame::Bulk(item.clone());
                    }
                }
            }
        }
        Some(_other) => return ReplyFrame::wrong_type_err(),
        None => (),
    }

    ReplyFrame::Null
}

#[cfg(test)]
mod tests {
    use crate::cmd::reply_frame::ReplyFrame;
    use crate::mem::db::Db;
    use crate::mem::list::index::index;
    use crate::mem::list::push_front::push_front;

    #[test]
    fn test_index() {
        let mut db = Db::new();
        let key = "mylist".to_owned();
        let reply = push_front(&mut db, key.clone(), vec![b"World".to_vec()]);
        assert_eq!(reply, ReplyFrame::Usize(1));
        let reply = push_front(&mut db, key.clone(), vec![b"Hello".to_vec()]);
        assert_eq!(reply, ReplyFrame::Usize(2));

        let reply = index(&db, &key, 0);
        assert_eq!(reply, ReplyFrame::Bulk(b"Hello".to_vec()));
        let reply = index(&db, &key, -1);
        assert_eq!(reply, ReplyFrame::Bulk(b"World".to_vec()));
        let reply = index(&db, &key, 3);
        assert_eq!(reply, ReplyFrame::Null);
    }
}
