// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use std::collections::HashMap;

use crate::cmd::Command;
use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::{list, Mem};
use crate::mem::bloom_filter::BloomFilterObject;
use crate::mem::hash::HashObject;
use crate::mem::hyper::HyperObject;
use crate::mem::list::ListObject;
use crate::mem::set::SetObject;
use crate::mem::string::StrObject;

pub type Db = HashMap<String, MemObject>;

#[derive(Debug, Clone)]
pub enum MemObject {
    // Core objects
    Str(StrObject),
    List(ListObject),
    Hash(HashObject),
    Set(SetObject),
    Hyper(HyperObject),

    // Stack objects
    BloomFilter(BloomFilterObject),
}

impl Mem {
    pub fn handle_db_command(&mut self, command: Command) -> ReplyFrame {
        match command {
            Command::Str(command) => self.handle_string_command(command),
            Command::List(command) => self.handle_list_command(command),
            Command::Hash(command) => self.handle_hash_command(command),
            Command::Set(command) => self.handle_set_command(command),
            Command::Bitmap(command) => self.handle_bitmap_command(command),
            Command::HyperLogLog(command) => self.handle_hyper_command(command),
            Command::Generic(command) => self.handle_generic_command(command),
            Command::BloomFilter(command) => self.handle_bloom_filter_command(command),
            _ => unreachable!(),
        }
    }
}

impl MemObject {
    pub fn to_reply_frame(&self) -> ReplyFrame {
        match self {
            Self::Str(s) => s.to_bulk(),
            Self::List(list_obj) => list::to_reply_frame(list_obj),
            _ => todo!(),
        }
    }
}
