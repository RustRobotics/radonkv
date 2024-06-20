// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::{Db, MemObject};

/// Returns the values of all specified keys.
///
/// For every key that does not hold a string value or does not exist, the special value nil is returned.
/// Because of this, the operation never fails.
///
/// Reply:
/// - Array reply: a list of values at the specified keys.
pub fn multi_get(db: &Db, keys: &[String]) -> ReplyFrame {
    let mut vec = Vec::with_capacity(keys.len());
    for key in keys {
        let reply = match db.get(key) {
            Some(MemObject::Str(old_str)) => old_str.to_bulk(),
            Some(_) => ReplyFrame::wrong_type_err(),
            None => ReplyFrame::Null,
        };
        vec.push(reply);
    }
    ReplyFrame::Array(vec)
}

#[cfg(test)]
mod tests {
    use crate::cmd::reply_frame::ReplyFrame;
    use crate::mem::db::Db;
    use crate::mem::string::multi_get::multi_get;
    use crate::mem::string::set::set;

    #[test]
    fn test_multi_get() {
        let mut db = Db::new();
        let reply = set(&mut db, "key1".to_owned(), b"Hello".to_vec());
        assert_eq!(reply, ReplyFrame::ok());
        let reply = set(&mut db, "key2".to_owned(), b"World".to_vec());
        assert_eq!(reply, ReplyFrame::ok());
        let reply = multi_get(
            &db,
            &[
                "key1".to_owned(),
                "key2".to_owned(),
                "nonexisting".to_owned(),
            ],
        );
        assert_eq!(
            reply,
            ReplyFrame::Array(vec![
                ReplyFrame::Bulk(b"Hello".to_vec()),
                ReplyFrame::Bulk(b"World".to_vec()),
                ReplyFrame::Null,
            ])
        );
    }
}
