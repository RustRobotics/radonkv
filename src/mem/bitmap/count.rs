// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::{Db, MemObject};

/// Count the number of set bits (population counting) in a string.
///
/// By default all the bytes contained in the string are examined.
/// It is possible to specify the counting operation only in an interval
/// passing the additional arguments start and end.
///
/// Reply:
/// - Integer reply: the number of bits set to 1.
pub fn count(db: &Db, key: &str, range: Option<(isize, isize)>) -> ReplyFrame {
    match db.get(key) {
        Some(MemObject::Str(old_bitmap)) => {
            // TODO(Shaohua): Add byte/bit flag.
            let count: usize = old_bitmap.count_bits(range, true);
            ReplyFrame::Usize(count)
        }
        Some(_) => ReplyFrame::wrong_type_err(),
        None => ReplyFrame::zero(),
    }
}

#[cfg(test)]
mod tests {
    use crate::cmd::reply_frame::ReplyFrame;
    use crate::mem::bitmap::count::count;
    use crate::mem::db::Db;
    use crate::mem::string::set::set;

    #[test]
    fn test_count() {
        let mut db = Db::new();
        let key = "mykey".to_owned();
        let reply = set(&mut db, key.clone(), b"foobar".to_vec());
        assert_eq!(reply, ReplyFrame::ok());
        let reply = count(&db, &key, None);
        assert_eq!(reply, ReplyFrame::Usize(26));

        let reply = count(&db, &key, Some((0, 0)));
        assert_eq!(reply, ReplyFrame::Usize(4));
        let reply = count(&db, &key, Some((1, 1)));
        assert_eq!(reply, ReplyFrame::Usize(6));
    }
}
