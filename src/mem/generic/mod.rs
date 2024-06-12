// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::generic::GenericCommand;
use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::Mem;

mod delete;
mod exists;
mod get_type;
mod random_key;
mod rename;

impl Mem {
    pub fn handle_generic_command(&mut self, command: GenericCommand) -> ReplyFrame {
        match command {
            GenericCommand::Delete(key, extra_keys) => {
                delete::delete(&mut self.db, &key, extra_keys)
            }
            GenericCommand::Exists(key, extra_keys) => {
                exists::exists(&mut self.db, &key, extra_keys)
            }
            GenericCommand::RandomKey(random_index) => {
                random_key::random_key(&self.db, random_index)
            }
            GenericCommand::Rename(key, new_key) => rename::rename(&mut self.db, key, new_key),
            GenericCommand::Type(key) => get_type::get_type(&self.db, &key),
        }
    }
}
