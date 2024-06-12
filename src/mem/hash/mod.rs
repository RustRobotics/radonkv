// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use std::collections::HashMap;

use crate::cmd::hash::HashCommand;
use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::Mem;

mod delete;
mod exists;
mod get;
mod get_all;
mod keys;
mod len;
mod set;
mod str_len;
mod values;

pub type HashObject = HashMap<String, Vec<u8>>;

impl Mem {
    pub fn handle_hash_command(&mut self, command: HashCommand) -> ReplyFrame {
        match command {
            HashCommand::Del(key, fields) => delete::delete(&mut self.db, &key, &fields),
            HashCommand::Exists(key, field) => exists::exists(&self.db, &key, &field),
            HashCommand::Get(key, field) => get::get(&self.db, &key, &field),
            HashCommand::GetAll(key) => get_all::get_all(&self.db, &key),
            HashCommand::Keys(key) => keys::keys(&self.db, &key),
            HashCommand::Len(key) => len::len(&self.db, &key),
            HashCommand::Set(key, pairs) => set::set(&mut self.db, key, pairs),
            HashCommand::StrLen(key, field) => str_len::str_len(&self.db, &key, &field),
            HashCommand::Values(key) => values::values(&self.db, &key),
        }
    }
}

pub fn to_reply_frame(_hash_object: &HashObject) -> ReplyFrame {
    todo!()
}
