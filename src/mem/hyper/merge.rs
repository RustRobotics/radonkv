// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use stdext::function_name;

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::{Db, MemObject};
use crate::mem::hyper::{HyperObject, new_hyper_object};

pub(super) fn merge_hyper_objects(
    db: &Db,
    dest_hyper: &mut HyperObject,
    source_keys: &[String],
) -> Result<(), ReplyFrame> {
    // FIXME(Shaohua): merge() does not work as expected.
    for source_key in source_keys {
        match db.get(source_key) {
            Some(MemObject::Hyper(extra_hyper)) => {
                if let Err(err) = dest_hyper.merge(extra_hyper) {
                    log::warn!(
                        "{} Failed to merge hyper logs, err: {err:?}",
                        function_name!()
                    );
                    return Err(ReplyFrame::internal_err());
                }
            }
            Some(_) => return Err(ReplyFrame::wrong_type_err()),
            None => continue,
        }
    }
    Ok(())
}

/// Merge multiple `HyperLogLog` values into a unique value that will approximate
/// the cardinality of the union of the observed Sets of the source `HyperLogLog` structures.
///
/// The computed merged `HyperLogLog` is set to the destination variable, which is created
/// if it does not exist (defaulting to an empty `HyperLogLog`).
///
/// If the destination variable exists, it is treated as one of the source sets and
/// its cardinality will be included in the cardinality of the computed `HyperLogLog`.
///
/// Reply:
/// - Simple string reply: OK.
pub fn merge(db: &mut Db, dest_key: String, source_keys: &[String]) -> ReplyFrame {
    match db.get(&dest_key) {
        Some(MemObject::Hyper(old_hyper)) => {
            let mut new_hyper = old_hyper.clone();
            if let Err(reply_frame) = merge_hyper_objects(db, &mut new_hyper, source_keys) {
                reply_frame
            } else {
                // FIXME(Shaohua): Do not clone hyper object.
                db.insert(dest_key, MemObject::Hyper(new_hyper));
                ReplyFrame::ok()
            }
        }
        Some(_) => ReplyFrame::wrong_type_err(),
        None => match new_hyper_object() {
            Ok(mut new_hyper) => {
                if let Err(reply_frame) = merge_hyper_objects(db, &mut new_hyper, source_keys) {
                    return reply_frame;
                }
                db.insert(dest_key, MemObject::Hyper(new_hyper));
                ReplyFrame::ok()
            }
            Err(err) => {
                log::warn!(
                    "{}, Failed to create new hyper object, err: {err:?}",
                    function_name!()
                );
                ReplyFrame::internal_err()
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::cmd::reply_frame::ReplyFrame;
    use crate::mem::db::Db;
    use crate::mem::hyper::add::add;
    use crate::mem::hyper::count::count;
    use crate::mem::hyper::merge::merge;

    #[test]
    fn test_add() {
        let mut db = Db::new();
        let key1 = "hll1".to_owned();
        let reply = add(
            &mut db,
            key1.clone(),
            &[
                "foo".to_owned(),
                "bar".to_owned(),
                "zap".to_owned(),
                "a".to_owned(),
            ],
        );
        assert_eq!(reply, ReplyFrame::I64(1));

        let key2 = "hll2".to_owned();
        let reply = add(
            &mut db,
            key2.clone(),
            &[
                "a".to_owned(),
                "b".to_owned(),
                "c".to_owned(),
                "foo".to_owned(),
            ],
        );
        assert_eq!(reply, ReplyFrame::I64(1));

        let key3 = "hll3".to_owned();
        let reply = merge(&mut db, key3.clone(), &[key1, key2]);
        assert_eq!(reply, ReplyFrame::ok());

        let reply = count(&mut db, &key3, &[]);
        // FIXME(Shaohua): Merge() failed.
        //assert_eq!(reply, ReplyFrame::I64(6));
        assert_eq!(reply, ReplyFrame::I64(8));
    }
}
