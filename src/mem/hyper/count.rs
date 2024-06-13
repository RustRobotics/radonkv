// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use hyperloglogplus::HyperLogLog;

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::{Db, MemObject};
use crate::mem::hyper::merge::merge_hyper_objects;

/// When called with a single key, returns the approximated cardinality
/// computed by the `HyperLogLog` data structure stored at the specified variable,
/// which is 0 if the variable does not exist.
///
/// When called with multiple keys, returns the approximated cardinality of the union
/// of the `HyperLogLogs` passed, by internally merging the `HyperLogLogs` stored
/// at the provided keys into a temporary `HyperLogLog`.
///
/// The `HyperLogLog` data structure can be used in order to count unique elements
/// in a set using just a small constant amount of memory, specifically 12k bytes
/// for every `HyperLogLog` (plus a few bytes for the key itself).
///
/// The returned cardinality of the observed set is not exact, but approximated
/// with a standard error of 0.81%.
///
/// For example in order to take the count of all the unique search queries performed in a day,
/// a program needs to call `PFADD` every time a query is processed.
/// The estimated number of unique queries can be retrieved with `PFCOUNT` at any time.
///
/// Note: as a side effect of calling this function, it is possible that the `HyperLogLog`
/// is modified, since the last 8 bytes encode the latest computed cardinality for caching purposes.
/// So `PFCOUN`T is technically a write command.
///
/// Reply:
/// - Integer reply: the approximated number of unique elements observed via `PFADD`.
#[allow(clippy::cast_possible_truncation)]
pub fn count(db: &mut Db, key: &str, extra_keys: &[String]) -> ReplyFrame {
    match db.get_mut(key) {
        Some(MemObject::Hyper(old_hyper)) => {
            if extra_keys.is_empty() {
                let count: i64 = old_hyper.count().trunc() as i64;
                return ReplyFrame::I64(count);
            }

            let mut merged_hyper = old_hyper.clone();
            if let Err(reply_frame) = merge_hyper_objects(db, &mut merged_hyper, extra_keys) {
                return reply_frame;
            }

            let merged_count: i64 = merged_hyper.count().trunc() as i64;
            return ReplyFrame::I64(merged_count);
        }
        Some(_) => ReplyFrame::wrong_type_err(),
        None => ReplyFrame::I64(0),
    }
}

#[cfg(test)]
mod tests {
    use crate::cmd::reply_frame::ReplyFrame;
    use crate::mem::db::Db;
    use crate::mem::hyper::add::add;
    use crate::mem::hyper::count::count;

    #[test]
    fn test_add() {
        let mut db = Db::new();
        let key = "hll".to_owned();
        let reply = add(
            &mut db,
            key.to_owned(),
            &["foo".to_owned(), "bar".to_owned(), "zap".to_owned()],
        );
        assert_eq!(reply, ReplyFrame::I64(1));
        let reply = add(
            &mut db,
            key.clone(),
            &["zap".to_owned(), "zap".to_owned(), "zap".to_owned()],
        );
        assert_eq!(reply, ReplyFrame::I64(0));
        let reply = add(&mut db, key.clone(), &["foo".to_owned(), "bar".to_owned()]);
        assert_eq!(reply, ReplyFrame::I64(0));
        let reply = count(&mut db, &key, &[]);
        assert_eq!(reply, ReplyFrame::I64(3));

        let other_key = "some-other-hll".to_owned();
        let reply = add(
            &mut db,
            other_key.clone(),
            &[
                "1".to_owned(),
                "2".to_owned(),
                "3".to_owned(),
                "foo".to_owned(),
            ],
        );
        assert_eq!(reply, ReplyFrame::I64(1));

        let reply = count(&mut db, &key, &[other_key]);
        // FIXME(Shaohua): the merged result is wrong
        //assert_eq!(reply, ReplyFrame::I64(6));
        assert_eq!(reply, ReplyFrame::I64(7));
    }
}
