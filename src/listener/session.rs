// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use stdext::function_name;

use crate::commands::{ListenerToDispatcherCmd, SessionToListenerCmd};
use crate::error::Error;
use crate::listener::Listener;
use crate::listener::types::SessionGroup;

impl Listener {
    pub(super) async fn handle_session_cmd(
        &mut self,
        cmd: SessionToListenerCmd,
    ) -> Result<(), Error> {
        log::debug!("{}", function_name!());
        match cmd {
            SessionToListenerCmd::Cmd(session_id, command) => {
                // Pass cmd to dispatcher
                let session_group = SessionGroup::new(self.id, session_id);
                let cmd = ListenerToDispatcherCmd::Cmd(session_group, command);
                log::debug!("{} proxy cmd from session to dispatcher, cmd: {cmd:?}", function_name!());
                self.dispatcher_sender.send(cmd).await?;
                Ok(())
            }
            SessionToListenerCmd::Disconnect(session_id) => {
                log::debug!("{} remove session: {session_id}", function_name!());
                self.session_senders.remove_entry(&session_id);
                Ok(())
            }
        }
    }
}
