// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use stdext::function_name;

use crate::commands::{DispatcherToListenerCmd, ListenerToSessionCmd};
use crate::error::{Error, ErrorKind};
use crate::listener::Listener;

impl Listener {
    pub(super) async fn handle_dispatcher_cmd(
        &mut self,
        cmd: DispatcherToListenerCmd,
    ) -> Result<(), Error> {
        log::debug!(
            "{}, proxy cmd from listener to session, cmd: {cmd:?}",
            function_name!()
        );
        let DispatcherToListenerCmd {
            session_group,
            reply_frames,
        } = cmd;
        assert_eq!(session_group.listener_id(), self.id);
        let session_id = session_group.session_id();
        match self.session_senders.get(&session_id) {
            Some(session_sender) => {
                let cmd = ListenerToSessionCmd {
                    session_id,
                    reply_frames,
                };
                Ok(session_sender.send(cmd).await?)
            }
            None => Err(Error::from_string(
                ErrorKind::ChannelError,
                format!("Failed to find session sender with id: {session_id}"),
            )),
        }
    }
}
