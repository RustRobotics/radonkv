// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::db::Db;
use crate::mem::string::sub_str::sub_str;

/// Returns the substring of the string value stored at key, determined by the offsets
/// start and end (both are inclusive).
///
/// Negative offsets can be used in order to provide an offset starting from the end of the string.
/// So -1 means the last character, -2 the penultimate and so forth.
///
/// The function handles out of range requests by limiting the resulting range
/// to the actual length of the string.
#[must_use]
#[inline]
pub fn get_range(db: &Db, key: &str, start: i64, end: i64) -> ReplyFrame {
    sub_str(db, key, start, end)
}