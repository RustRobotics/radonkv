// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use std::collections::hash_map::Entry;

use bytes::Bytes;

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::{Db, MemObject};

/// Removes and returns the last elements of the list stored at key.
///
/// By default, the command pops a single element from the end of the list.
/// When provided with the optional count argument, the reply will consist of up to count elements,
/// depending on the list's length.
pub fn pop_back(db: &mut Db, key: String, count: Option<usize>) -> ReplyFrame {
    match db.entry(key) {
        Entry::Occupied(mut occupied) => match occupied.get_mut() {
            MemObject::Str(_) => ReplyFrame::wrong_type_err(),
            MemObject::List(old_list) => {
                let real_count: usize = count.unwrap_or(1).min(old_list.len() - 1);
                let mut items = Vec::new();
                for _i in 0..real_count {
                    let item = if let Some(item) = old_list.pop_back() {
                        ReplyFrame::Bulk(Bytes::from(item))
                    } else {
                        ReplyFrame::EmptyBulk
                    };
                    items.push(item);
                }
                ReplyFrame::Array(items)
            }
        }
        Entry::Vacant(_) => ReplyFrame::Null,
    }
}