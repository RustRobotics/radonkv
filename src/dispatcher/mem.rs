// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::commands::{DispatcherToListenerCmd, MemToDispatcherCmd};
use crate::dispatcher::Dispatcher;
use crate::error::{Error, ErrorKind};

impl Dispatcher {
    pub(super) async fn handle_mem_cmd(&mut self, cmd: MemToDispatcherCmd) -> Result<(), Error> {
        // Send command to listener.
        let listener_id = cmd.session_group.listener_id();
        if let Some(listener_sender) = self.listener_senders.get(&listener_id) {
            let cmd = DispatcherToListenerCmd::Reply(cmd.session_group, cmd.frame);
            Ok(listener_sender.send(cmd).await?)
        } else {
            Err(Error::from_string(
                ErrorKind::ChannelError,
                format!("Failed to find listener with id: {listener_id}"),
            ))
        }
    }
}
