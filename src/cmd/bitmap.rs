// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::Command;
use crate::cmd::parse::{ParseCommandError, Parser};

#[derive(Debug, Clone)]
pub enum BitmapCommand {
    Get(String, usize),
}

impl BitmapCommand {
    pub(super) fn parse(
        cmd_name: &str,
        parser: &mut Parser,
    ) -> Result<Option<Command>, ParseCommandError> {
        let bitmap_cmd = match cmd_name {
            "lindex" => {
                let key = parser.next_string()?;
                let offset = parser.next_usize()?;
                Self::Get(key, offset)
            }
            _ => return Ok(None),
        };
        Ok(Some(Command::Bitmap(bitmap_cmd)))
    }
}
