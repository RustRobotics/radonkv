// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use std::collections::HashMap;

use bytes::Bytes;

use crate::cmd::Command;
use crate::cmd::frame::Frame;
use crate::error::Error;
use crate::mem::Mem;

pub type Db = HashMap<String, MemObject>;

#[derive(Debug, Clone)]
pub enum MemObject {
    Str(Bytes),
    List(String),
}

impl Mem {
    pub fn handle_db_command(&mut self, command: Command) -> Result<Frame, Error> {
        match command {
            Command::Str(command) => self.handle_string_command(command),
            Command::List(command) => self.handle_list_command(command),
        }
    }
}