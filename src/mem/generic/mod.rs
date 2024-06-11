// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::generic::GenericCommand;
use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::Mem;

mod delete;
mod get_type;
mod rename;


impl Mem {
    pub fn handle_generic_command(&mut self, command: GenericCommand) -> ReplyFrame {
        match command {
            GenericCommand::Delete(keys) => delete::delete(&mut self.db, keys),
            GenericCommand::Rename(key, new_key) => rename::rename(&mut self.db, key, new_key),
            GenericCommand::Type(key) => get_type::get_type(&self.db, &key),
        }
    }
}
