// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::CommandCategory;
use crate::commands::{DispatcherToMemCmd, ListenerToDispatcherCmd};
use crate::dispatcher::Dispatcher;
use crate::error::Error;

impl Dispatcher {
    pub(super) async fn handle_listener_cmd(
        &mut self,
        cmd: ListenerToDispatcherCmd,
    ) -> Result<(), Error> {
        match cmd {
            ListenerToDispatcherCmd::Cmd(session_group, command) => match command.category() {
                CommandCategory::Mem => {
                    // Dispatch to mem module
                    let cmd = DispatcherToMemCmd {
                        session_group,
                        command,
                    };
                    Ok(self.mem_sender.send(cmd).await?)
                }
                CommandCategory::System => {
                    // Dispatch to system module
                    todo!()
                }
                CommandCategory::Cluster => {
                    // Dispatch to cluster module
                    todo!()
                }
                CommandCategory::Storage => {
                    // Dispatch to storage module
                    todo!()
                }
            },
        }
    }
}
