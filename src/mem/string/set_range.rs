// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use bytes::BufMut;

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::{Db, MemObject};
use crate::mem::string::consts::STRING_TOO_LONG_ERR;
use crate::mem::string::StrObject;
use crate::mem::util::check_string_length;

/// Overwrites part of the string stored at key, starting at the specified offset,
/// for the entire length of value.
///
/// If the offset is larger than the current length of the string at key,
/// the string is padded with zero-bytes to make offset fit.
/// Non-existing keys are considered as empty strings, so this command will make sure
/// it holds a string large enough to be able to set value at offset.
///
/// Reply:
/// - Integer reply: the length of the string after it was modified by the command.
#[allow(clippy::cast_sign_loss)]
pub fn set_range(db: &mut Db, key: String, offset: isize, value: Vec<u8>) -> ReplyFrame {
    if offset < 0 {
        return ReplyFrame::ConstError("offset is out of range");
    }
    // TODO(Shaohua): Replace with util::prune_index
    let offset_usize = offset as usize;

    if let Some(old_value) = db.get_mut(&key) {
        let old_value = match old_value {
            MemObject::Str(s) => &mut s.vec,
            _ => return ReplyFrame::wrong_type_err(),
        };
        if value.is_empty() {
            ReplyFrame::Usize(old_value.len())
        } else if !check_string_length(offset_usize, value.len()) {
            ReplyFrame::ConstErrorWithErr(STRING_TOO_LONG_ERR)
        } else {
            // FIXME(Shaohua): merge two parts of vector
            old_value.put_slice(&value);
            ReplyFrame::Usize(old_value.len())
        }
    } else {
        if value.is_empty() {
            return ReplyFrame::zero();
        }

        if !check_string_length(offset_usize, value.len()) {
            return ReplyFrame::ConstErrorWithErr(STRING_TOO_LONG_ERR);
        }

        let mut s = StrObject::with_length(offset_usize);
        s.append(value);
        let len = s.len();
        db.insert(key, MemObject::Str(s));
        ReplyFrame::Usize(len)
    }
}

#[cfg(test)]
mod tests {
    use crate::cmd::reply_frame::ReplyFrame;
    use crate::mem::db::Db;
    use crate::mem::string::get::get;
    use crate::mem::string::set::set;
    use crate::mem::string::set_range::set_range;

    #[test]
    fn test_set_range() {
        let mut db = Db::new();
        let key1 = "key1".to_owned();
        let reply = set(&mut db, key1.clone(), b"Hello World".to_vec());
        assert_eq!(reply, ReplyFrame::ok());
        let reply = set_range(&mut db, key1.clone(), 6, b"Redis".to_vec());
        assert_eq!(reply, ReplyFrame::Usize(11));
        let reply = get(&db, &key1);
        assert_eq!(reply, ReplyFrame::Bulk(b"Hello Redis".to_vec()));

        let key2 = "key2".to_owned();
        let reply = set_range(&mut db, key2.clone(), 6, b"Redis".to_vec());
        assert_eq!(reply, ReplyFrame::Usize(11));
        let reply = get(&db, &key2);
        assert_eq!(reply, ReplyFrame::Bulk(b"Redis".to_vec()));
    }
}
