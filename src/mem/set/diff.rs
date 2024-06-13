// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::{Db, MemObject};

/// Returns the members of the set resulting from the difference between the first set
/// and all the successive sets.
///
/// Reply:
/// - Array reply: a list with members of the resulting set.
pub fn diff(db: &Db, keys: &[String]) -> ReplyFrame {
    let mut new_set = match db.get(&keys[0]) {
        Some(MemObject::Set(old_set)) => old_set.clone(),
        Some(_) => return ReplyFrame::wrong_type_err(),
        None => return ReplyFrame::EmptyArray,
    };

    for key in &keys[1..] {
        match db.get(key) {
            Some(MemObject::Set(old_set)) => {
                new_set = new_set.difference(old_set).cloned().collect();
            }
            Some(_) => return ReplyFrame::wrong_type_err(),
            None => continue,
        }
    }
    let vec = new_set.into_iter().map(ReplyFrame::bulk).collect();
    ReplyFrame::Array(vec)
}

#[cfg(test)]
mod tests {
    use crate::cmd::reply_frame::ReplyFrame;
    use crate::mem::db::Db;
    use crate::mem::set::add::add;
    use crate::mem::set::diff::diff;

    #[test]
    fn test_union() {
        let mut db = Db::new();
        let key1 = "key1".to_owned();
        let reply = add(
            &mut db,
            key1.clone(),
            vec![b"a".to_vec(), b"b".to_vec(), b"c".to_vec()],
        );
        assert_eq!(reply, ReplyFrame::Usize(3));
        let key2 = "key2".to_owned();
        let reply = add(
            &mut db,
            key2.clone(),
            vec![b"c".to_vec(), b"d".to_vec(), b"e".to_vec()],
        );
        assert_eq!(reply, ReplyFrame::Usize(3));
        let reply = diff(&db, &[key1, key2]);
        assert_eq!(
            reply,
            ReplyFrame::Array(vec![
                ReplyFrame::Bulk(b"a".to_vec()),
                ReplyFrame::Bulk(b"b".to_vec()),
            ])
        )
    }
}
