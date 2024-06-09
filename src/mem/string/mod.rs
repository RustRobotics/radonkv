// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::frame::Frame;
use crate::cmd::string::StringCommand;
use crate::mem::Mem;

mod get;
mod set;
mod strlen;

impl Mem {
    pub fn handle_string_command(&mut self, command: StringCommand) -> Frame {
        match command {
            StringCommand::Get(key) => get::get(&self.db, &key),
            StringCommand::Set(key, value) => set::set(&mut self.db, key, value),
            StringCommand::StrLen(key) => strlen::strlen(&self.db, &key),
        }
    }
}
