// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::{Db, MemObject};

/// Returns all the members of the set value stored at key.
///
/// This has the same effect as running `SINTER` with one argument key.
///
/// RESP2 Reply:
/// - Array reply: an array with all the members of the set.
pub fn members(db: &Db, key: &str) -> ReplyFrame {
    match db.get(key) {
        Some(MemObject::Set(old_set)) => {
            // NOTE(Shaohua): Sort members.
            let mut vec: Vec<Vec<u8>> = old_set.iter().cloned().collect();
            vec.sort_unstable();
            let vec: Vec<_> = vec
                .into_iter()
                .map(|member| ReplyFrame::bulk(member))
                .collect();
            ReplyFrame::Array(vec)
        }
        Some(_) => ReplyFrame::wrong_type_err(),
        None => ReplyFrame::EmptyArray,
    }
}

#[cfg(test)]
mod tests {
    use crate::cmd::reply_frame::ReplyFrame;
    use crate::mem::db::Db;
    use crate::mem::set::add::add;
    use crate::mem::set::members::members;

    #[test]
    fn test_add() {
        let mut db = Db::new();
        let key = "myset".to_owned();
        let reply = add(&mut db, key.clone(), vec![b"Hello".to_vec()]);
        assert_eq!(reply, ReplyFrame::one());
        let reply = add(&mut db, key.clone(), vec![b"World".to_vec()]);
        assert_eq!(reply, ReplyFrame::one());
        let reply = members(&db, &key);
        assert_eq!(
            reply,
            ReplyFrame::Array(vec![
                ReplyFrame::Bulk(b"Hello".to_vec()),
                ReplyFrame::Bulk(b"World".to_vec()),
            ])
        )
    }
}
