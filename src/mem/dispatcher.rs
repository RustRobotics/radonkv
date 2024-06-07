// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use stdext::function_name;

use crate::commands::DispatcherToMemCmd;
use crate::error::Error;
use crate::mem::Mem;

impl Mem {
    pub(super) async fn handle_dispatcher_cmd(&mut self, cmd: DispatcherToMemCmd) -> Result<(), Error> {
        log::info!("{}, cmd: {cmd:?}", function_name!());

        Ok(())
    }
}