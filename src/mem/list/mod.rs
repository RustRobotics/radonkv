// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::frame::Frame;
use crate::cmd::list::ListCommand;
use crate::error::Error;
use crate::mem::Mem;

impl Mem {
    #[allow(clippy::needless_pass_by_value)]
    pub fn handle_list_command(&mut self, _command: ListCommand) -> Result<Frame, Error> {
        todo!()
    }
}
