// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::reply_frame::ReplyFrame;
use crate::cmd::string::StringCommand;
use crate::mem::db::MemObject;
use crate::mem::Mem;

mod append;
mod consts;
mod get;
mod get_del;
mod get_range;
mod get_set;
mod len;
mod set;
mod set_range;
mod sub_str;

#[derive(Debug, Clone)]
pub struct StrObject {
    pub(crate) vec: Vec<u8>,
}

impl StrObject {
    #[must_use]
    #[inline]
    pub fn with_length(len: usize) -> Self {
        Self { vec: vec![0; len] }
    }

    #[must_use]
    #[inline]
    pub fn from_bytes(vec: Vec<u8>) -> MemObject {
        MemObject::Str(Self { vec })
    }

    #[must_use]
    #[inline]
    pub fn to_bulk(&self) -> ReplyFrame {
        ReplyFrame::Bulk(self.vec.clone())
    }

    #[must_use]
    #[inline]
    pub fn into_bulk(self) -> ReplyFrame {
        ReplyFrame::Bulk(self.vec)
    }

    #[must_use]
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[must_use]
    #[inline]
    pub fn len(&self) -> usize {
        self.vec.len()
    }

    #[inline]
    pub fn append(&mut self, mut value: Vec<u8>) {
        self.vec.append(&mut value);
    }

    #[inline]
    pub fn clear(&mut self) {
        self.vec.clear();
    }
}

impl Mem {
    pub fn handle_string_command(&mut self, command: StringCommand) -> ReplyFrame {
        match command {
            StringCommand::Append(key, value) => append::append(&mut self.db, key, value),
            StringCommand::Get(key) => get::get(&self.db, &key),
            StringCommand::GetDel(key) => get_del::get_del(&mut self.db, &key),
            StringCommand::GetRange(key, start, end) => {
                get_range::get_range(&self.db, &key, start, end)
            }
            StringCommand::GetSet(key, value) => get_set::get_set(&mut self.db, key, value),
            StringCommand::Set(key, value) => set::set(&mut self.db, key, value),
            StringCommand::SetRange(key, offset, value) => {
                set_range::set_range(&mut self.db, key, offset, value)
            }
            StringCommand::StrLen(key) => len::len(&self.db, &key),
            StringCommand::SubStr(key, start, end) => sub_str::sub_str(&self.db, &key, start, end),
        }
    }
}
