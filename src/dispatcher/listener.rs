// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use stdext::function_name;

use crate::cmd::CommandCategory;
use crate::commands::{DispatcherToMemCmd, DispatcherToServerCmd, ListenerToDispatcherCmd};
use crate::dispatcher::Dispatcher;
use crate::error::Error;

impl Dispatcher {
    pub(super) fn handle_listener_cmd(
        &mut self,
        cmd: ListenerToDispatcherCmd,
    ) -> Result<(), Error> {
        log::debug!("{}", function_name!());
        match cmd {
            ListenerToDispatcherCmd::Cmd(session_group, command) => match command.category() {
                CommandCategory::Mem => {
                    // Dispatch to mem module
                    let cmd = DispatcherToMemCmd {
                        session_group,
                        command,
                    };
                    log::debug!(
                        "{} proxy cmd from listener to mem, cmd: {cmd:?}",
                        function_name!()
                    );
                    Ok(self.mem_sender.send(cmd)?)
                }
                CommandCategory::Server => {
                    // Dispatch to server module.
                    let cmd = DispatcherToServerCmd {
                        session_group,
                        command,
                    };
                    log::debug!(
                        "{} proxy cmd from listener to server, cmd: {cmd:?}",
                        function_name!()
                    );
                    Ok(self.server_sender.send(cmd)?)
                }
                _ => unimplemented!(),
            },
        }
    }
}
