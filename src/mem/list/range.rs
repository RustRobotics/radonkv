// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::{Db, MemObject};
use crate::util::prune_range::prune_range;

/// Returns the specified elements of the list stored at key.
///
/// The offsets start and stop are zero-based indexes, with 0 being the first element of the list
/// (the head of the list), 1 being the next element and so on.
//
// These offsets can also be negative numbers indicating offsets starting at the end of the list.
// For example, -1 is the last element of the list, -2 the penultimate, and so on.
pub fn range(db: &Db, key: &str, start: isize, end: isize) -> ReplyFrame {
    match db.get(key) {
        Some(MemObject::List(list)) => {
            if let Some((start, end)) = prune_range(list.len(), start, end) {
                let mut sub_list = Vec::new();
                // FIXME(Shaohua): Check list range error.
                for item in list.iter().take(end + 1).skip(start) {
                    sub_list.push(ReplyFrame::Bulk(item.clone()));
                }
                ReplyFrame::Array(sub_list)
            } else {
                ReplyFrame::EmptyArray
            }
        }
        Some(_other) => ReplyFrame::wrong_type_err(),
        None => ReplyFrame::Null,
    }
}