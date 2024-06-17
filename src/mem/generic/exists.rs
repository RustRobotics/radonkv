// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::Db;

/// Returns if key exists.
///
/// The user should be aware that if the same existing key is mentioned in the arguments multiple times,
/// it will be counted multiple times.
/// So if `somekey` exists, `EXISTS somekey somekey` will return 2.
///
/// Reply:
/// - Integer reply: the number of keys that exist from those specified as arguments.
pub fn exists(db: &Db, keys: &[String]) -> ReplyFrame {
    let count = keys
        .iter()
        .filter(|key| db.contains_key(key.as_str()))
        .count();
    ReplyFrame::Usize(count)
}

#[cfg(test)]
mod tests {
    use crate::cmd::reply_frame::ReplyFrame;
    use crate::mem::db::Db;
    use crate::mem::generic::exists::exists;
    use crate::mem::string::set::set;

    #[test]
    fn test_exists() {
        let mut db = Db::new();
        let key1 = "key1".to_owned();
        let reply = set(&mut db, key1.clone(), b"Hello".to_vec());
        assert_eq!(reply, ReplyFrame::ok());
        let reply = exists(&db, &[key1.clone()]);
        assert_eq!(reply, ReplyFrame::one());
        let no_such_key = "nosuchkey".to_owned();
        let reply = exists(&db, &[no_such_key.clone()]);
        assert_eq!(reply, ReplyFrame::zero());
        let key2 = "key2".to_owned();
        let reply = set(&mut db, key2.clone(), b"World".to_vec());
        assert_eq!(reply, ReplyFrame::ok());
        let reply = exists(&db, &[key1, key2, no_such_key]);
        assert_eq!(reply, ReplyFrame::Usize(2));
    }
}
