// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use std::collections::HashMap;

use crate::cmd::hash::HashCommand;
use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::Mem;

mod len;

pub type HashObject = HashMap<String, Vec<u8>>;

impl Mem {
    pub fn handle_hash_command(&mut self, command: HashCommand) -> ReplyFrame {
        match command {
            HashCommand::Len(key) => len::len(&self.db, &key),
        }
    }
}

pub fn to_reply_frame(_hash_object: &HashObject) -> ReplyFrame {
    todo!()
}
