// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::list::RelativePosition;
use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::{Db, MemObject};

/// Inserts element in the list stored at key either before or after the reference value pivot.
///
/// When key does not exist, it is considered an empty list and no operation is performed.
///
/// An error is returned when key exists but does not hold a list value.
///
/// One of the following reply:
/// - Integer reply: the list length after a successful insert operation.
/// - Integer reply: 0 when the key doesn't exist.
/// - Integer reply: -1 when the pivot wasn't found.
pub fn insert(
    db: &mut Db,
    key: &str,
    position: RelativePosition,
    pivot: &[u8],
    element: Vec<u8>,
) -> ReplyFrame {
    match db.get_mut(key) {
        Some(MemObject::List(old_list)) => {
            old_list
                .iter()
                .position(|value| value == pivot)
                .map_or_else(ReplyFrame::minus_one, |index| {
                    // TODO(Shaohua): Simplify
                    let index = if position == RelativePosition::Before {
                        index
                    } else {
                        index + 1
                    };
                    let mut tail_list = old_list.split_off(index);
                    old_list.push_back(element);
                    old_list.append(&mut tail_list);
                    ReplyFrame::Usize(old_list.len())
                })
        }
        Some(_) => ReplyFrame::wrong_type_err(),
        None => ReplyFrame::zero(),
    }
}

#[cfg(test)]
mod tests {
    use crate::cmd::list::RelativePosition;
    use crate::cmd::reply_frame::ReplyFrame;
    use crate::mem::db::Db;
    use crate::mem::list::insert::insert;
    use crate::mem::list::push_back::push_back;
    use crate::mem::list::range::range;

    #[test]
    fn test_insert() {
        let mut db = Db::new();
        let key = "mylist".to_owned();
        let reply = push_back(&mut db, key.clone(), vec![b"Hello".to_vec()]);
        assert_eq!(reply, ReplyFrame::Usize(1));
        let reply = push_back(&mut db, key.clone(), vec![b"World".to_vec()]);
        assert_eq!(reply, ReplyFrame::Usize(2));
        let reply = insert(
            &mut db,
            key.clone(),
            RelativePosition::Before,
            b"World".to_vec(),
            b"Three".to_vec(),
        );
        assert_eq!(reply, ReplyFrame::Usize(3));

        let reply = range(&db, &key, 0, -1);
        assert_eq!(
            reply,
            ReplyFrame::Array(vec![
                ReplyFrame::Bulk(b"Hello".to_vec()),
                ReplyFrame::Bulk(b"Three".to_vec()),
                ReplyFrame::Bulk(b"World".to_vec()),
            ])
        );
    }
}
