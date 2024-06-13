// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use std::collections::hash_map::Entry;

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::bloom_filter::BloomFilterObject;
use crate::mem::db::{Db, MemObject};

/// Adds one or more items to a Bloom filter.
///
/// Returns one of these replies:
/// - Integer reply - where "1" means that the item has been added successfully,
///   and "0" means that such item was already added to the filter (which could be wrong)
/// - [] on error (invalid arguments, wrong key type, etc.) and also when the filter is full
pub fn add(db: &mut Db, key: String, item: &String) -> ReplyFrame {
    // TODO(Shaohua): Replace `&String` with `&str` type.
    match db.entry(key) {
        Entry::Occupied(mut occupied) => match occupied.get_mut() {
            MemObject::BloomFilter(old_filter) => {
                let already_set = old_filter.check_and_set(item);
                ReplyFrame::from_bool(!already_set)
            }
            _ => ReplyFrame::wrong_type_err(),
        },
        Entry::Vacant(vacant) => {
            let mut new_filter = BloomFilterObject::new();
            let already_set = new_filter.check_and_set(item);
            debug_assert!(!already_set);
            vacant.insert(MemObject::BloomFilter(new_filter));
            ReplyFrame::one()
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
        let reply = add(&mut db, key.clone(), &"item1".to_owned());
        assert_eq!(reply, ReplyFrame::one());
        let reply = add(&mut db, key, &"item1".to_owned());
        assert_eq!(reply, ReplyFrame::zero());
    }
}
