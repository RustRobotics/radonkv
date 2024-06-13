// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::{Db, MemObject};

/// Determines whether a given item was added to a Bloom filter.
///
/// Returns one of these replies:
/// - Integer reply, where 1 means that, with high probability, item was already
///   added to the filter, and 0 means that key does not exist or that item
///   had not been added to the filter.
/// - [] on error (invalid arguments, wrong key type, etc.)
pub fn exists(db: &Db, key: &str, item: &String) -> ReplyFrame {
    match db.get(key) {
        Some(MemObject::BloomFilter(old_filter)) => {
            let is_set = old_filter.check(item);
            ReplyFrame::from_bool(is_set)
        }
        Some(_) => ReplyFrame::wrong_type_err(),
        None => ReplyFrame::zero(),
    }
}

#[cfg(test)]
mod tests {
    use crate::cmd::reply_frame::ReplyFrame;
    use crate::mem::bloom_filter::add::add;
    use crate::mem::bloom_filter::exists::exists;
    use crate::mem::db::Db;

    #[test]
    fn test_exists() {
        let mut db = Db::new();
        let key = "bf1".to_owned();
        let reply = add(&mut db, key.clone(), &"item1".to_owned());
        assert_eq!(reply, ReplyFrame::one());
        let reply = exists(&db, &key, &"item1".to_owned());
        assert_eq!(reply, ReplyFrame::one());
        let reply = exists(&db, &key, &"item2".to_owned());
        assert_eq!(reply, ReplyFrame::zero());
    }
}
