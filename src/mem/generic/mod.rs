// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::generic::GenericCommand;
use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::generic::flush_db::flush_db;
use crate::mem::Mem;

mod db_size;
pub mod delete;
pub mod exists;
mod flush_db;
pub mod get_type;
pub mod random_key;
pub mod rename;

impl Mem {
    pub fn handle_generic_command(&mut self, command: GenericCommand) -> ReplyFrame {
        match command {
            GenericCommand::DbSize => db_size::db_size(&self.db),
            GenericCommand::Delete(keys) => delete::delete(&mut self.db, &keys),
            GenericCommand::Exists(keys) => exists::exists(&self.db, &keys),
            GenericCommand::RandomKey(random_index) => {
                random_key::random_key(&self.db, random_index)
            }
            GenericCommand::Rename(key, new_key) => rename::rename(&mut self.db, &key, new_key),
            GenericCommand::Type(key) => get_type::get_type(&self.db, &key),
            GenericCommand::FlushDb(is_sync) => flush_db(&mut self.db, is_sync),
        }
    }
}
