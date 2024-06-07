// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::commands::{ListenerToDispatcherCmd, SessionToListenerCmd};
use crate::error::Error;
use crate::listener::Listener;
use crate::listener::types::SessionGid;

impl Listener {
    pub(super) async fn handle_session_cmd(&mut self, cmd: SessionToListenerCmd) -> Result<(), Error> {
        match cmd {
            SessionToListenerCmd::Cmd(session_id, command) => {
                // Pass cmd to dispatcher
                let cmd = ListenerToDispatcherCmd::Cmd(SessionGid::new(self.id, session_id), command);
                self.dispatcher_sender.send(cmd).await?;
                Ok(())
            }
            SessionToListenerCmd::Disconnect(session_id) => {
                self.session_senders.remove_entry(&session_id);
                Ok(())
            }
        }
    }
}