// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::Command;
use crate::cmd::parse::{ParseCommandError, Parser};

#[derive(Debug, Clone)]
pub enum ListCommand {
    LGet(String),
    LLen(String),
}

impl ListCommand {
    pub(super) fn parse(
        cmd_name: &str,
        _parser: &Parser,
    ) -> Result<Option<Command>, ParseCommandError> {
        match cmd_name {
            "lget" => todo!(),
            "llen" => todo!(),
            _ => Ok(None),
        }
    }
}
