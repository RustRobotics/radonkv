// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use std::collections::hash_map::Entry;

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::{Db, MemObject};
use crate::mem::string::StrObject;

/// Sets or clears the bit at offset in the string value stored at key.
///
/// The bit is either set or cleared depending on value, which can be either 0 or 1.
///
/// When key does not exist, a new string value is created.
/// The string is grown to make sure it can hold a bit at offset.
/// The offset argument is required to be greater than or equal to 0, and smaller than 2^32
/// (this limits bitmaps to 512MB).
/// When the string at key is grown, added bits are set to 0.
///
/// Reply:
/// - Integer reply: the original bit value stored at offset.
pub fn set(db: &mut Db, key: String, offset: usize, value: bool) -> ReplyFrame {
    match db.entry(key) {
        Entry::Occupied(mut occupied) => match occupied.get_mut() {
            MemObject::Str(old_bitmap) => {
                old_bitmap.grow_to_fit_bits(offset, false);
                let old_value = old_bitmap.get_bit(offset).unwrap_or(false);
                old_bitmap.set_bit(offset, value);
                ReplyFrame::Usize(old_value.into())
            }
            _ => ReplyFrame::wrong_type_err(),
        },
        Entry::Vacant(vacant) => {
            let mut new_bitmap = StrObject::from_bits(offset, false);
            new_bitmap.set_bit(offset, value);
            vacant.insert(MemObject::Str(new_bitmap));
            ReplyFrame::zero()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cmd::reply_frame::ReplyFrame;
    use crate::mem::bitmap::set::set;
    use crate::mem::db::Db;

    #[test]
    fn test_set() {
        let mut db = Db::new();
        let key = "mykey".to_owned();
        let reply = set(&mut db, key.clone(), 7, true);
        assert_eq!(reply, ReplyFrame::zero());
        let reply = set(&mut db, key.clone(), 7, false);
        assert_eq!(reply, ReplyFrame::one());
    }
}
