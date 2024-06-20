// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::{Db, MemObject};

/// Inserts specified values at the head of the list stored at key,
/// only if key already exists and holds a list.
///
/// In contrary to `LPUSH`, no operation will be performed when key does not yet exist.
///
/// Reply:
// - Integer reply: the length of the list after the push operation.
pub fn push_front_exist(db: &mut Db, key: &str, values: Vec<Vec<u8>>) -> ReplyFrame {
    match db.get_mut(key) {
        Some(MemObject::List(old_list)) => {
            for value in values {
                old_list.push_front(value);
            }
            ReplyFrame::Usize(old_list.len())
        }
        Some(_) => ReplyFrame::wrong_type_err(),
        None => ReplyFrame::zero(),
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
        let reply = push_front(&mut db, key.clone(), vec![b"World".to_vec()]);
        assert_eq!(reply, ReplyFrame::Usize(1));

        let reply = push_front_exist(&mut db, &key, vec![b"Hello".to_vec()]);
        assert_eq!(reply, ReplyFrame::Usize(2));
        let reply = push_front_exist(&mut db, "myotherlist", vec![b"Hello".to_vec()]);
        assert_eq!(reply, ReplyFrame::Usize(0));

        let reply = range(&db, &key, 0, -1);
        assert_eq!(
            reply,
            ReplyFrame::Array(vec![
                ReplyFrame::Bulk(b"Hello".to_vec()),
                ReplyFrame::Bulk(b"World".to_vec()),
            ])
        );

        let reply = range(&db, "myotherlist", 0, -1);
        assert_eq!(reply, ReplyFrame::EmptyArray);
    }
}
