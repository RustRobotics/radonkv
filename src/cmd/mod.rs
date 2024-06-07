// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::list::ListCommand;
use crate::cmd::string::StringCommand;

pub mod frame;
mod parse;
pub mod string;
pub mod list;

#[derive(Debug, Clone)]
pub enum Command {
    Str(StringCommand),
    List(ListCommand),
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub enum CommandCategory {
    #[default]
    Mem,
    System,
    Cluster,
    Storage,
}

impl Command {
    #[must_use]
    pub const fn category(&self) -> CommandCategory {
        match self {
            Self::Str(_) | Self::List(_) => CommandCategory::Mem,
        }
    }

    #[must_use]
    #[inline]
    pub fn is_mem(&self) -> bool {
        self.category() == CommandCategory::Mem
    }
}
