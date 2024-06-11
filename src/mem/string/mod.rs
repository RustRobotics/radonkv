// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::reply_frame::ReplyFrame;
use crate::cmd::string::StringCommand;
use crate::mem::db::MemObject;
use crate::mem::Mem;

mod get;
mod set;
mod len;
mod append;
mod get_del;
mod get_set;
mod sub_str;
mod get_range;
mod set_range;
mod consts;

#[derive(Debug, Clone)]
pub enum StrObject {
    Integer(i64),
    Vec(Vec<u8>),
}

impl StrObject {
    #[must_use]
    #[inline]
    pub fn with_length(len: usize) -> Self {
        Self::Vec(vec![0; len])
    }

    #[must_use]
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn from_bytes(bytes: Vec<u8>) -> MemObject {
        MemObject::Str(Self::Vec(bytes))
    }

    pub fn append(&mut self, mut bytes: Vec<u8>) {
        match self {
            Self::Integer(_integer) => todo!(),
            Self::Vec(vec) => {
                vec.append(&mut bytes);
            }
        }
    }

    #[must_use]
    #[inline]
    pub fn to_bulk(&self) -> ReplyFrame {
        match self {
            Self::Integer(_integer) => todo!(),
            Self::Vec(vec) => {
                ReplyFrame::Bulk(vec.clone())
            }
        }
    }

    #[must_use]
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[must_use]
    #[inline]
    pub fn len(&self) -> usize {
        match self {
            // TODO(Shaohua):
            Self::Integer(_) => 8,
            Self::Vec(vec) => vec.len(),
        }
    }
}

impl Mem {
    pub fn handle_string_command(&mut self, command: StringCommand) -> ReplyFrame {
        match command {
            StringCommand::Append(key, value) => append::append(&mut self.db, key, value),
            StringCommand::Get(key) => get::get(&self.db, &key),
            StringCommand::GetDel(key) => get_del::get_del(&mut self.db, &key),
            StringCommand::GetRange(key, start, end) => get_range::get_range(&self.db, &key, start, end),
            StringCommand::GetSet(key, value) => get_set::get_set(&mut self.db, key, value),
            StringCommand::Set(key, value) => set::set(&mut self.db, key, value),
            StringCommand::SetRange(key, offset, value) => set_range::set_range(&mut self.db, key, offset, value),
            StringCommand::StrLen(key) => len::len(&self.db, &key),
            StringCommand::SubStr(key, start, end) => sub_str::sub_str(&self.db, &key, start, end),
        }
    }
}
