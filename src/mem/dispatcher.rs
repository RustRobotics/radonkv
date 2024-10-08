// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use stdext::function_name;

use crate::commands::{DispatcherToMemCmd, MemToDispatcherCmd};
use crate::error::Error;
use crate::mem::Mem;

impl Mem {
    pub(super) async fn handle_dispatcher_cmd(
        &mut self,
        cmd: DispatcherToMemCmd,
    ) -> Result<(), Error> {
        log::debug!("{}, cmd: {cmd:?}", function_name!());
        let DispatcherToMemCmd {
            session_group,
            commands,
        } = cmd;
        let reply_frames = self.handle_db_commands(commands);
        let reply_cmd = MemToDispatcherCmd {
            session_group,
            reply_frames,
        };
        log::debug!(
            "{} send cmd to dispatcher, cmd: {reply_cmd:?}",
            function_name!()
        );
        self.dispatcher_sender.send(reply_cmd).await?;
        Ok(())
    }
}
