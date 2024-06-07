// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use stdext::function_name;

use crate::commands::ListenerToSessionCmd;
use crate::error::Error;
use crate::session::Session;

impl Session {
    pub(crate) async fn handle_listener_cmd(
        &mut self,
        cmd: ListenerToSessionCmd,
    ) -> Result<(), Error> {
        match cmd {
            ListenerToSessionCmd::Reply(session_id, frame) => {
                assert_eq!(session_id, self.id);
                log::info!("{}, frame: {frame:?}", function_name!());
                Ok(self.send_frame_to_client(frame).await?)
            }
        }
    }
}
