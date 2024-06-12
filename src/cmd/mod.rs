// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::conn::ConnectManagementCommand;
use crate::cmd::generic::GenericCommand;
use crate::cmd::hash::HashCommand;
use crate::cmd::list::ListCommand;
use crate::cmd::string::StringCommand;

pub mod conn;
pub mod frame;
pub mod generic;
pub mod hash;
pub mod list;
mod parse;
pub mod reply_frame;
pub mod string;

#[derive(Debug, Clone)]
pub enum Command {
    Str(StringCommand),
    List(ListCommand),
    Hash(HashCommand),
    Generic(GenericCommand),
    ConnManagement(ConnectManagementCommand),
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub enum CommandCategory {
    #[default]
    Mem,
    System,
    Cluster,
    Storage,
    // Handle commands in session module.
    Session,
}

impl Command {
    #[must_use]
    pub const fn category(&self) -> CommandCategory {
        match self {
            Self::Str(_) | Self::List(_) | Self::Generic(_) | Self::Hash(_) => CommandCategory::Mem,
            Self::ConnManagement(_) => CommandCategory::Session,
        }
    }

    #[must_use]
    #[inline]
    pub fn is_mem(&self) -> bool {
        self.category() == CommandCategory::Mem
    }
}

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use crate::cmd::Command;

    #[test]
    fn test_command() {
        assert_eq!(size_of::<Command>(), 72);
    }
}
