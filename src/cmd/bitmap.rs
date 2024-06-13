// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::Command;
use crate::cmd::parse::{ParseCommandError, Parser};

#[derive(Debug, Clone)]
pub enum BitmapCommand {
    Get(String, usize),
    Set(String, usize, bool),
    FromBytes(String, Vec<u8>),
}

impl BitmapCommand {
    pub(super) fn parse(
        cmd_name: &str,
        parser: &mut Parser,
    ) -> Result<Option<Command>, ParseCommandError> {
        let bitmap_cmd = match cmd_name {
            "getbit" => {
                let key = parser.next_string()?;
                let offset = parser.next_usize()?;
                Self::Get(key, offset)
            }
            "setbit" => {
                let key = parser.next_string()?;
                let offset = parser.next_usize()?;
                let value = parser.next_i32()? != 0;
                Self::Set(key, offset, value)
            }
            "bitfromstr" => {
                let key = parser.next_string()?;
                let value = parser.next_bytes()?;
                Self::FromBytes(key, value)
            }
            _ => return Ok(None),
        };
        Ok(Some(Command::Bitmap(bitmap_cmd)))
    }
}
