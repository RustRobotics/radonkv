// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::frame::Frame;
use crate::cmd::string::StringCommand;
use crate::error::Error;
use crate::mem::db::MemObject;
use crate::mem::Mem;

impl Mem {
    pub fn handle_string_command(&mut self, command: StringCommand) -> Result<Frame, Error> {
        match command {
            StringCommand::Get(key) => {
                match self.db.get(&key) {
                    Some(MemObject::Str(value)) => {
                        Ok(Frame::Bulk(value.clone()))
                    }
                    Some(_other) => {
                        Ok(Frame::Error("Object type mismatch, expected string".to_owned()))
                    }
                    None => Ok(Frame::Error("Not found".to_owned())),
                }
            }
            StringCommand::Set(key, value) => {
                self.db.insert(key, MemObject::Str(value));
                Ok(Frame::ok())
            }
            StringCommand::Len(key) => {
                match self.db.get(&key) {
                    Some(MemObject::Str(value)) => {
                        Ok(Frame::Integer(value.len() as i64))
                    }
                    Some(_other) => {
                        Ok(Frame::Error("Object type mismatch, expected string".to_owned()))
                    }
                    None => Ok(Frame::Error("Not found".to_owned())),
                }
            }
        }
    }
}