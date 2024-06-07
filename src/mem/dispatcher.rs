// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::commands::{DispatcherToMemCmd, MemToDispatcherCmd};
use crate::error::Error;
use crate::mem::Mem;

impl Mem {
    pub(super) async fn handle_dispatcher_cmd(
        &mut self,
        cmd: DispatcherToMemCmd,
    ) -> Result<(), Error> {
        let DispatcherToMemCmd {
            session_group,
            command,
        } = cmd;
        let reply_frame = self.handle_db_command(command)?;
        let reply_cmd = MemToDispatcherCmd {
            session_group,
            frame: reply_frame,
        };
        Ok(self.dispatcher_sender.send(reply_cmd).await?)
    }
}
