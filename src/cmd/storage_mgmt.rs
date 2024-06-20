// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::parse::{ParseCommandError, Parser};
use crate::cmd::Command;

#[derive(Debug, Clone)]
pub enum StorageManagementCommand {
    BackgroundWriteAof,
    BackgroundSave,
    Save,
}

impl StorageManagementCommand {
    pub fn parse(
        cmd_name: &str,
        _parser: &mut Parser,
    ) -> Result<Option<Command>, ParseCommandError> {
        let storage_cmd = match cmd_name {
            "bgrewriteaof" => Self::BackgroundWriteAof,
            "bgsave" => Self::BackgroundSave,
            "save" => Self::Save,
            _ => return Ok(None),
        };
        Ok(Some(Command::StorageManagement(storage_cmd)))
    }
}
