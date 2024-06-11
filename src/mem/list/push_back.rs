// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use std::collections::hash_map::Entry;

use bytes::Bytes;

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::{Db, MemObject};
use crate::mem::list::ListObject;

/// Insert all the specified values at the tail of the list stored at key.
///
/// If key does not exist, it is created as empty list before performing the push operation.
/// When key holds a value that is not a list, an error is returned.
//
// It is possible to push multiple elements using a single command call
// just specifying multiple arguments at the end of the command.
// Elements are inserted one after the other to the tail of the list, from the leftmost element
// to the rightmost element.
//
// So for instance the command `RPUSH mylist a b c` will result into a list containing
// `a` as first element, `b` as second element and `c` as third element.
pub fn push_back(db: &mut Db, key: String, values: Vec<Bytes>) -> ReplyFrame {
    match db.entry(key) {
        Entry::Occupied(mut occupied) => match occupied.get_mut() {
            MemObject::Str(_) => ReplyFrame::wrong_type_err(),
            MemObject::List(old_list) => {
                for value in values {
                    old_list.push_back(value.to_vec());
                }
                ReplyFrame::Usize(old_list.len())
            }
        }
        Entry::Vacant(vacant) => {
            let len = values.len();
            // Keep order of items in values.
            let list = ListObject::from_iter(values.into_iter().map(|item| item.to_vec()));
            vacant.insert(MemObject::List(list));
            ReplyFrame::Usize(len)
        }
    }
}