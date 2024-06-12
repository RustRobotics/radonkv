// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use std::collections::HashMap;

use crate::cmd::hash::{ExtraValues, HashCommand};
use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::Mem;

mod get;
mod get_all;
mod len;
mod set;

pub type HashObject = HashMap<String, Vec<u8>>;

impl Mem {
    pub fn handle_hash_command(&mut self, command: HashCommand) -> ReplyFrame {
        match command {
            HashCommand::Get(key, field) => get::get(&self.db, &key, &field),
            HashCommand::GetAll(key) => get_all::get_all(&self.db, &key),
            HashCommand::Len(key) => len::len(&self.db, &key),
            HashCommand::Set(key, field, value, extra_values) => {
                set::set(&mut self.db, key, field, value, extra_values)
            }
        }
    }
}

pub fn to_reply_frame(_hash_object: &HashObject) -> ReplyFrame {
    todo!()
}

fn append_to_hash(
    hash_object: &mut HashObject,
    field: String,
    value: Vec<u8>,
    extra_values: ExtraValues,
) -> usize {
    let mut count = 0;
    if hash_object.insert(field, value).is_none() {
        count += 1;
    }
    if let Some(extra_values) = extra_values {
        for (field, value) in extra_values.into_iter() {
            if hash_object.insert(field, value).is_none() {
                count += 1;
            }
        }
    }
    count
}
