// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use std::collections::HashMap;

use crate::cmd::Command;
use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::list::ListObject;
use crate::mem::Mem;
use crate::mem::string::StrObject;

pub type Db = HashMap<String, MemObject>;

#[derive(Debug, Clone)]
pub enum MemObject {
    Str(StrObject),
    List(ListObject),
}

impl Mem {
    pub fn handle_db_command(&mut self, command: Command) -> ReplyFrame {
        match command {
            Command::Str(command) => self.handle_string_command(command),
            Command::List(command) => self.handle_list_command(command),
            Command::Generic(command) => self.handle_generic_command(command),
        }
    }
}
