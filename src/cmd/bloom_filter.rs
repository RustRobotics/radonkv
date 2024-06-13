// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::Command;
use crate::cmd::parse::{ParseCommandError, Parser};

#[derive(Debug, Clone)]
pub enum BloomFilterCommand {
    Add(String, String),
    MultiAdd(String, Vec<String>),
    Exists(String, String),
    MultiExists(String, Vec<String>),
    Len(String),
}

impl BloomFilterCommand {
    pub fn parse(
        cmd_name: &str,
        parser: &mut Parser,
    ) -> Result<Option<Command>, ParseCommandError> {
        let bloom_filter_cmd = match cmd_name {
            "bf.add" => {
                let key = parser.next_string()?;
                let item = parser.next_string()?;
                Self::Add(key, item)
            }
            "bf.madd" => {
                let key = parser.next_string()?;
                let items = parser.remaining_strings()?;
                Self::MultiAdd(key, items)
            }
            "bf.exists" => {
                let key = parser.next_string()?;
                let item = parser.next_string()?;
                Self::Exists(key, item)
            }
            "BF.MEXISTS" => {
                let key = parser.next_string()?;
                let items = parser.remaining_strings()?;
                Self::MultiExists(key, items)
            }
            "bf.card" => {
                let key = parser.next_string()?;
                Self::Len(key)
            }
            _ => return Ok(None),
        };
        Ok(Some(Command::BloomFilter(bloom_filter_cmd)))
    }
}
