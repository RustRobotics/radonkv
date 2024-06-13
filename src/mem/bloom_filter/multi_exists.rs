// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::{Db, MemObject};

/// Determines whether one or more items were added to a Bloom filter.
///
/// Returns one of these replies:
/// - Array reply of Integer reply - where "1" means that, with high probability,
///   item was already added to the filter, and "0" means that key does not exist
///   or that item was definitely not added to the filter.
/// - [] on error (invalid arguments, wrong key type, etc.)
pub fn multi_exists(db: &Db, key: &str, items: &[String]) -> ReplyFrame {
    match db.get(key) {
        Some(MemObject::BloomFilter(old_filter)) => {
            let mut vec = Vec::new();
            for item in items {
                let is_set = old_filter.check(item);
                vec.push(ReplyFrame::from_bool(is_set));
            }
            ReplyFrame::Array(vec)
        }
        Some(_) => ReplyFrame::wrong_type_err(),
        None => ReplyFrame::zero(),
    }
}

#[cfg(test)]
mod tests {
    use crate::cmd::reply_frame::ReplyFrame;
    use crate::mem::bloom_filter::multi_add::multi_add;
    use crate::mem::bloom_filter::multi_exists::multi_exists;
    use crate::mem::db::Db;

    #[test]
    fn test_multi_exists() {
        let mut db = Db::new();
        let key = "bf".to_owned();
        let reply = multi_add(
            &mut db,
            key.clone(),
            &["item1".to_owned(), "item2".to_owned()],
        );
        assert_eq!(
            reply,
            ReplyFrame::Array(vec![ReplyFrame::one(), ReplyFrame::one(),])
        );
        let reply = multi_exists(
            &db,
            &key,
            &["item1".to_owned(), "item2".to_owned(), "item3".to_owned()],
        );
        assert_eq!(
            reply,
            ReplyFrame::Array(vec![
                ReplyFrame::one(),
                ReplyFrame::one(),
                ReplyFrame::zero()
            ])
        );
    }
}
