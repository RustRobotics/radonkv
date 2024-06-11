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
        None => ReplyFrame::Null,
    }
}

#[cfg(test)]
mod tests {
    use crate::mem::db::Db;
    use crate::mem::list::push_back::push_back;

    #[test]
    fn test_range() {
        let mut db = Db::new();
        let key = "mylist";
        push_back(&mut db, key.to_owned(), vec);
    }
}