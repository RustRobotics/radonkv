// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::list::ListCommand;
use crate::cmd::string::StringCommand;

pub mod frame;
mod parse;
mod string;
mod list;

#[derive(Debug, Clone)]
pub enum Command {
    Str(StringCommand),
    List(ListCommand),
}
