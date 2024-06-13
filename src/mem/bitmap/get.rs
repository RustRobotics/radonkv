// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::{Db, MemObject};

///
/// Returns the bit value at offset in the string value stored at key.
///
/// When offset is beyond the string length, the string is assumed to be a contiguous space
/// with 0 bits.
/// When key does not exist it is assumed to be an empty string, so offset is always out of range
/// and the value is also assumed to be a contiguous space with 0 bits.
///
/// The bit value stored at offset, one of the following:
/// - Integer reply: 0.
/// - Integer reply: 1.
pub fn get(db: &Db, key: &str, offset: usize) -> ReplyFrame {
    match db.get(key) {
        Some(MemObject::Bitmap(old_bitmap)) => {
            let is_set = old_bitmap.get(offset).unwrap_or(false);
            ReplyFrame::Usize(is_set as usize)
        }
        Some(_) => ReplyFrame::wrong_type_err(),
        None => ReplyFrame::zero(),
    }
}

#[cfg(test)]
mod tests {
    use crate::cmd::reply_frame::ReplyFrame;
    use crate::mem::bitmap::get::get;
    use crate::mem::db::Db;

    #[test]
    fn test_get() {
        let mut db = Db::new();
        let key = "mykey".to_owned();
        let reply = set(&mut db, key.clone(), 7, 1);
        assert_eq!(reply, ReplyFrame::zero());
        let reply = get(&mut db, &key, 0);
        assert_eq!(reply, ReplyFrame::zero());
        let reply = get(&mut db, &key, 7);
        assert_eq!(reply, ReplyFrame::one());
        let reply = get(&mut db, &key, 100);
        assert_eq!(reply, ReplyFrame::zero());
    }
}
