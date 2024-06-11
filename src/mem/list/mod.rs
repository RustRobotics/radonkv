// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use std::collections::LinkedList;

use crate::cmd::frame::Frame;
use crate::cmd::list::ListCommand;
use crate::mem::Mem;

mod len;

pub type ListObject = LinkedList<Vec<u8>>;

impl Mem {
    #[allow(clippy::needless_pass_by_value)]
    pub fn handle_list_command(&mut self, command: ListCommand) -> Frame {
        match command {
            ListCommand::Len(key) => len::len(&self.db, &key),
        }
    }
}
