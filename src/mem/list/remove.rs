// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use std::cmp::Ordering;
use std::collections::LinkedList;

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::{Db, MemObject};

/// Removes the first count occurrences of elements equal to element from the list stored at key.
///
/// The count argument influences the operation in the following ways:
/// - count > 0: Remove elements equal to element moving from head to tail.
/// - count < 0: Remove elements equal to element moving from tail to head.
/// - count = 0: Remove all elements equal to element.
///
/// For example, `LREM list -2 "hello"` will remove the last two occurrences of "hello" in the list stored at list.
///
/// Note that non-existing keys are treated like empty lists, so when key does not exist,
/// the command will always return 0.
///
/// Reply:
/// - Integer reply: the number of removed elements.
#[allow(clippy::cast_sign_loss)]
pub fn remove(db: &mut Db, key: &str, count: isize, element: &[u8]) -> ReplyFrame {
    match db.get_mut(key) {
        Some(MemObject::List(old_list)) => {
            // TODO(Shaohua): Simplify operation.
            let mut new_list = LinkedList::new();
            let mut num_removed = 0;
            match count.cmp(&0) {
                Ordering::Equal => {
                    while let Some(value) = old_list.pop_front() {
                        if value == element {
                            num_removed += 1;
                        } else {
                            new_list.push_back(value);
                        }
                    }
                }
                Ordering::Greater => {
                    let mut count = count as usize;
                    while let Some(value) = old_list.pop_front() {
                        if value == element && count > 0 {
                            num_removed += 1;
                            count -= 1;
                        } else {
                            new_list.push_back(value);
                        }
                    }
                }

                Ordering::Less => {
                    let mut count = (-count) as usize;
                    while let Some(value) = old_list.pop_back() {
                        if value == element && count > 0 {
                            count -= 1;
                            num_removed += 1;
                        } else {
                            new_list.push_front(value);
                        }
                    }
                }
            }

            *old_list = new_list;
            ReplyFrame::Usize(num_removed)
        }
        Some(_) => ReplyFrame::wrong_type_err(),
        None => ReplyFrame::zero(),
    }
}

#[cfg(test)]
mod tests {
    use crate::cmd::reply_frame::ReplyFrame;
    use crate::mem::db::Db;
    use crate::mem::list::push_back::push_back;
    use crate::mem::list::range::range;
    use crate::mem::list::remove::remove;

    #[test]
    fn test_remove() {
        let mut db = Db::new();
        let key = "mylist".to_owned();
        let reply = push_back(&mut db, key.clone(), vec![b"hello".to_vec()]);
        assert_eq!(reply, ReplyFrame::Usize(1));
        let reply = push_back(&mut db, key.clone(), vec![b"hello".to_vec()]);
        assert_eq!(reply, ReplyFrame::Usize(2));
        let reply = push_back(&mut db, key.clone(), vec![b"foo".to_vec()]);
        assert_eq!(reply, ReplyFrame::Usize(3));
        let reply = push_back(&mut db, key.clone(), vec![b"hello".to_vec()]);
        assert_eq!(reply, ReplyFrame::Usize(4));

        let reply = remove(&mut db, &key, -2, b"hello");
        assert_eq!(reply, ReplyFrame::Usize(2));
        let reply = range(&mut db, &key, 0, -1);
        assert_eq!(
            reply,
            ReplyFrame::Array(vec![
                ReplyFrame::Bulk(b"hello".to_vec()),
                ReplyFrame::Bulk(b"foo".to_vec())
            ])
        );
    }
}
