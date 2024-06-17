// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::Db;

/// Removes the specified keys. A key is ignored if it does not exist.
///
/// Reply:
/// - Integer reply: the number of keys that were removed.
pub fn delete(db: &mut Db, keys: &[String]) -> ReplyFrame {
    let mut count: usize = 0;
    for key in keys {
        if db.remove(key).is_some() {
            count += 1;
        }
    }

    ReplyFrame::Usize(count)
}

#[cfg(test)]
mod tests {
    use crate::cmd::reply_frame::ReplyFrame;
    use crate::mem::db::Db;
    use crate::mem::generic::delete::delete;
    use crate::mem::string::set;

    #[test]
    fn test_delete() {
        let mut db = Db::new();
        let reply = set::set(&mut db, "key1".to_owned(), b"Hello".to_vec());
        assert_eq!(reply, ReplyFrame::ok());
        let reply = set::set(&mut db, "key2".to_owned(), b"World".to_vec());
        assert_eq!(reply, ReplyFrame::ok());
        let reply = delete(
            &mut db,
            &["key1".to_owned(), "key2".to_owned(), "key3".to_owned()],
        );
        assert_eq!(reply, ReplyFrame::Usize(2));
    }
}
