// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use stdext::function_name;

use crate::cmd::Command;
use crate::commands::{
    DispatcherToClusterCmd, DispatcherToListenerCmd, DispatcherToMemCmd, DispatcherToServerCmd,
    DispatcherToStorageCmd, ListenerToDispatcherCmd,
};
use crate::dispatcher::Dispatcher;
use crate::error::{Error, ErrorKind};
use crate::listener::types::ListenerId;

impl Dispatcher {
    pub(super) async fn handle_listener_cmd(
        &mut self,
        cmd: ListenerToDispatcherCmd,
    ) -> Result<(), Error> {
        log::debug!("{}", function_name!());
        match cmd {
            ListenerToDispatcherCmd::Cmd(session_group, command) => match command {
                Command::ConnManagement(_) => unreachable!(),
                Command::ClusterManagement(command) => {
                    // Dispatch to server module.
                    let cmd = DispatcherToClusterCmd {
                        session_group,
                        command,
                    };
                    log::debug!(
                        "{} proxy cmd from listener to cluster, cmd: {cmd:?}",
                        function_name!()
                    );
                    Ok(self.cluster_sender.send(cmd).await?)
                }
                Command::StorageManagement(command) => {
                    // Dispatch to storage module.
                    let cmd = DispatcherToStorageCmd {
                        session_group,
                        command,
                    };
                    log::debug!(
                        "{} proxy cmd from listener to storage, cmd: {cmd:?}",
                        function_name!()
                    );
                    Ok(self.storage_sender.send(cmd).await?)
                }
                Command::ServerManagement(command) => {
                    // Dispatch to server module.
                    let cmd = DispatcherToServerCmd {
                        session_group,
                        command,
                    };
                    log::debug!(
                        "{} proxy cmd from listener to server, cmd: {cmd:?}",
                        function_name!()
                    );
                    Ok(self.server_sender.send(cmd).await?)
                }
                _ => {
                    // Dispatch to mem module
                    let cmd = DispatcherToMemCmd {
                        session_group,
                        command,
                    };
                    log::debug!(
                        "{} proxy cmd from listener to mem, cmd: {cmd:?}",
                        function_name!()
                    );
                    Ok(self.mem_sender.send(cmd).await?)
                }
            },
        }
    }

    pub(super) async fn send_cmd_to_listener(
        &mut self,
        listener_id: ListenerId,
        cmd: DispatcherToListenerCmd,
    ) -> Result<(), Error> {
        if let Some(listener_sender) = self.listener_senders.get(&listener_id) {
            listener_sender.send(cmd).await?;
            Ok(())
        } else {
            Err(Error::from_string(
                ErrorKind::ChannelError,
                format!("Failed to find listener with id: {listener_id}"),
            ))
        }
    }
}
