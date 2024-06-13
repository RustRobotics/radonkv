// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use std::collections::HashSet;

use crate::cmd::reply_frame::ReplyFrame;
use crate::cmd::set::SetCommand;
use crate::mem::Mem;

mod add;
mod len;
mod members;

pub type SetObject = HashSet<Vec<u8>>;

impl Mem {
    pub fn handle_set_command(&mut self, command: SetCommand) -> ReplyFrame {
        match command {
            SetCommand::Add(key, members) => add::add(&mut self.db, key, members),
            SetCommand::Len(key) => len::len(&self.db, &key),
            SetCommand::Members(key) => members::members(&self.db, &key),
        }
    }
}
