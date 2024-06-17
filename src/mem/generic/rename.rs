// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::Db;

/// Renames key to `new_key`.
///
/// It returns an error when key does not exist.
/// If `new_key` already exists it is overwritten, when this happens
/// `RENAME` executes an implicit `DEL` operation, so if the deleted key
/// contains a very big value it may cause high latency even if RENAME itself
/// is usually a constant-time operation.
///
/// In Cluster mode, both key and `new_key` must be in the same hash slot,
/// meaning that in practice only keys that have the same hashtag can be reliably
/// renamed in cluster.
///
/// Reply:
/// - Simple string reply: OK.
pub fn rename(db: &mut Db, key: &str, new_key: String) -> ReplyFrame {
    db.remove(key)
        .map_or_else(ReplyFrame::no_such_key, |value| {
            db.insert(new_key, value);
            ReplyFrame::ok()
        })
}

#[cfg(test)]
mod tests {
    use crate::cmd::reply_frame::ReplyFrame;
    use crate::mem::db::Db;
    use crate::mem::generic::rename::rename;
    use crate::mem::string::get::get;
    use crate::mem::string::set::set;

    #[test]
    fn test_rename() {
        let mut db = Db::new();
        let key = "mykey".to_owned();
        let reply = set(&mut db, key.clone(), b"Hello".to_vec());
        assert_eq!(reply, ReplyFrame::ok());
        let other_key = "myotherkey".to_owned();
        let reply = rename(&mut db, &key, other_key.clone());
        assert_eq!(reply, ReplyFrame::ok());
        let reply = get(&db, &other_key);
        assert_eq!(reply, ReplyFrame::bulk(b"Hello".to_vec()));
    }
}
