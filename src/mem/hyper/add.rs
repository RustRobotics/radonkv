// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use std::collections::hash_map::Entry;
use std::hash::RandomState;

use hyperloglogplus::{HyperLogLog, HyperLogLogPlus};
use stdext::function_name;

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::{Db, MemObject};

/// Adds all the element arguments to the `HyperLogLog` data structure stored
/// at the variable name specified as first argument.
///
/// As a side effect of this command the `HyperLogLog` internals may be updated
/// to reflect a different estimation of the number of unique items added so far
/// (the cardinality of the set).
///
/// If the approximated cardinality estimated by the `HyperLogLog` changed after
/// executing the command, `PFADD` returns 1, otherwise 0 is returned.
/// The command automatically creates an empty `HyperLogLog` structure if
/// the specified key does not exist.
///
/// To call the command without elements but just the variable name is valid,
/// this will result into no operation performed if the variable already exists,
/// or just the creation of the data structure if the key does not exist (in the latter case 1 is returned).
///
/// One of the following reply:
/// - Integer reply: 1 if at least one `HyperLogLog` internal register was altered.
/// - Integer reply: 0 if no `HyperLogLog` internal registers were altered.
pub fn add(db: &mut Db, key: String, elements: &[String]) -> ReplyFrame {
    match db.entry(key) {
        Entry::Occupied(mut occupied) => match occupied.get_mut() {
            MemObject::Hyper(old_hyper) => {
                let current_count = old_hyper.count().trunc();
                for element in elements {
                    old_hyper.insert(element);
                }
                let new_count = old_hyper.count().trunc();

                ReplyFrame::I64(i64::from(new_count > current_count))
            }
            _ => ReplyFrame::wrong_type_err(),
        },
        Entry::Vacant(vacant) => match HyperLogLogPlus::new(18, RandomState::new()) {
            Ok(mut new_hyper) => {
                for element in elements {
                    new_hyper.insert(element);
                }
                vacant.insert(MemObject::Hyper(new_hyper));
                ReplyFrame::I64(1)
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

    #[test]
    fn test_add() {
        let mut db = Db::new();
        let key = "hll".to_owned();
        let reply = add(
            &mut db,
            key.to_owned(),
            &[
                "a".to_owned(),
                "b".to_owned(),
                "c".to_owned(),
                "d".to_owned(),
                "e".to_owned(),
                "f".to_owned(),
                "g".to_owned(),
            ],
        );
        assert_eq!(reply, ReplyFrame::I64(1));
        let reply = count(&mut db, &key, &[]);
        assert_eq!(reply, ReplyFrame::I64(7));
    }
}
