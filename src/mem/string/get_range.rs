// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::frame::Frame;
use crate::mem::db::Db;
use crate::mem::string::sub_str::sub_str;

#[must_use]
#[inline]
pub fn get_range(db: &Db, key: &str, start: i64, end: i64) -> Frame {
    sub_str(db, key, start, end)
}