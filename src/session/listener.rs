// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use stdext::function_name;

use crate::commands::{ListenerToSessionCmd, SessionToListenerCmd};
use crate::error::Error;
use crate::session::Session;

impl Session {
    pub(super) async fn handle_listener_cmd(
        &mut self,
        cmd: ListenerToSessionCmd,
    ) -> Result<(), Error> {
        log::debug!(
            "{} got reply cmd from listener, cmd: {cmd:?}",
            function_name!()
        );
        assert_eq!(cmd.session_id, self.id);
        self.send_frames_to_client(cmd.reply_frames).await
    }

    #[allow(clippy::unused_async)]
    pub(super) async fn send_disconnect_to_listener(&mut self) -> Result<(), Error> {
        self.listener_sender
            .send(SessionToListenerCmd::Disconnect(self.id))
            .await?;
        Ok(())
    }
}
