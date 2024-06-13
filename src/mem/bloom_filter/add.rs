// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use std::collections::hash_map::Entry;

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::bloom_filter::new_bloom_filter;
use crate::mem::db::{Db, MemObject};

/// Adds one or more items to a Bloom filter.
///
/// Returns one of these replies:
/// - Integer reply - where "1" means that the item has been added successfully,
///   and "0" means that such item was already added to the filter (which could be wrong)
/// - [] on error (invalid arguments, wrong key type, etc.) and also when the filter is full

pub fn add(db: &mut Db, key: String, items: &[String]) -> ReplyFrame {
    match db.entry(key) {
        Entry::Occupied(mut occupied) => match occupied.get_mut() {
            MemObject::BloomFilter(old_filter) => {
                let mut vec = Vec::new();
                for item in items {
                    let is_set = old_filter.check_and_set(item);
                    vec.push(ReplyFrame::Usize(if is_set { 0 } else { 1 }));
                }
                ReplyFrame::Array(vec)
            }
            _ => ReplyFrame::wrong_type_err(),
        },
        Entry::Vacant(vacant) => {
            let mut new_filter = new_bloom_filter();
            let mut vec = Vec::new();
            for item in items {
                let is_set = new_filter.check_and_set(item);
                vec.push(ReplyFrame::Usize(if is_set { 0 } else { 1 }));
            }
            vacant.insert(MemObject::BloomFilter(new_filter));
            ReplyFrame::Array(vec)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cmd::reply_frame::ReplyFrame;
    use crate::mem::bloom_filter::add::add;
    use crate::mem::db::Db;

    #[test]
    fn test_add() {
        let mut db = Db::new();
        let key = "bf".to_owned();
        let reply = add(
            &mut db,
            key,
            &["item1".to_owned(), "item2".to_owned(), "item2".to_owned()],
        );
        assert_eq!(
            reply,
            ReplyFrame::Array(vec![
                ReplyFrame::one(),
                ReplyFrame::one(),
                ReplyFrame::zero(),
            ])
        )
    }
}
