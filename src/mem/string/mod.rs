// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use bytes::Bytes;

use crate::cmd::frame::Frame;
use crate::cmd::string::StringCommand;
use crate::mem::Mem;

mod get;
mod set;
mod strlen;
mod append;

#[derive(Debug, Clone)]
pub enum StrObject {
    Integer(i64),
    Vec(Vec<u8>),
}

impl StrObject {
    #[must_use]
    #[inline]
    pub fn from_bytes(bytes: Bytes) -> Self {
        Self::Vec(bytes.to_vec())
    }

    pub fn append(&mut self, bytes: Bytes) {
        match self {
            StrObject::Integer(_integer) => todo!(),
            StrObject::Vec(vec) => {
                let mut bytes_vec = bytes.to_vec();
                vec.append(&mut bytes_vec);
            }
        }
    }

    pub fn to_bytes(&self) -> Bytes {
        match self {
            StrObject::Integer(_integer) => todo!(),
            StrObject::Vec(vec) => {
                Bytes::copy_from_slice(&vec)
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
            StrObject::Integer(_) => 8,
            StrObject::Vec(vec) => vec.len(),
        }
    }
}

impl Mem {
    pub fn handle_string_command(&mut self, command: StringCommand) -> Frame {
        match command {
            StringCommand::Append(key, value) => append::append(&mut self.db, key, value),
            StringCommand::Get(key) => get::get(&self.db, &key),
            StringCommand::Set(key, value) => set::set(&mut self.db, key, value),
            StringCommand::StrLen(key) => strlen::strlen(&self.db, &key),
        }
    }
}
