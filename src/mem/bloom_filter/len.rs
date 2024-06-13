// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::{Db, MemObject};

/// Returns the cardinality of a Bloom filter - number of items that were added to a Bloom filter
/// and detected as unique (items that caused at least one bit to be set in at least one sub-filter).
///
///
///
/// Returns one of these replies:
/// - Integer reply - the number of items that were added to this Bloom filter
///   and detected as unique (items that caused at least one bit to be set in
///   at least one sub-filter), or 0 when key does not exist.
/// - [] on error (invalid arguments, wrong key type, etc.)
pub fn len(db: &Db, key: &str) -> ReplyFrame {
    match db.get(key) {
        Some(MemObject::BloomFilter(old_filter)) => ReplyFrame::Usize(old_filter.len()),
        Some(_) => ReplyFrame::wrong_type_err(),
        None => ReplyFrame::zero(),
    }
}

#[cfg(test)]
mod tests {
    use crate::cmd::reply_frame::ReplyFrame;
    use crate::mem::bloom_filter::add::add;
    use crate::mem::bloom_filter::len::len;
    use crate::mem::db::Db;

    #[test]
    fn test_len() {
        let mut db = Db::new();
        let key = "bf1".to_owned();
        let reply = add(&mut db, key.clone(), &["item_foo".to_owned()]);
        assert_eq!(reply, ReplyFrame::Array(vec![ReplyFrame::one()]));
        let reply = len(&db, &key);
        assert_eq!(reply, ReplyFrame::one());
        let reply = len(&db, "bf-new");
        assert_eq!(reply, ReplyFrame::zero());
    }
}
