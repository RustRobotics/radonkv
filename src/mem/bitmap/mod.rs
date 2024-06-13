// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use bit_vec::BitVec;

use crate::cmd::bitmap::BitmapCommand;
use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::Mem;

mod from_bytes;
mod get;
mod set;

// TODO(Shaohua): Convert Vec<u8> as BitVec.
pub type BitmapObject = BitVec;

impl Mem {
    #[allow(clippy::needless_pass_by_value)]
    pub fn handle_bitmap_command(&mut self, command: BitmapCommand) -> ReplyFrame {
        match command {
            BitmapCommand::Get(key, offset) => get::get(&self.db, &key, offset),
            BitmapCommand::Set(key, offset, value) => set::set(&mut self.db, key, offset, value),
            BitmapCommand::FromBytes(key, value) => {
                from_bytes::from_bytes(&mut self.db, key, &value)
            }
        }
    }
}

pub fn to_reply_frame(_bitmap_object: &BitmapObject) -> ReplyFrame {
    todo!()
}
