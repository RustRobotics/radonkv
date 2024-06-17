// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use stdext::function_name;

use crate::commands::{DispatcherToListenerCmd, StorageToDispatcherCmd};
use crate::dispatcher::Dispatcher;
use crate::error::Error;

impl Dispatcher {
    pub(super) fn handle_storage_cmd(&mut self, cmd: StorageToDispatcherCmd) -> Result<(), Error> {
        // Send command to listener.
        log::debug!(
            "{}, proxy cmd from storage to listener, cmd: {cmd:?}",
            function_name!()
        );
        let listener_id = cmd.session_group.listener_id();
        let cmd = DispatcherToListenerCmd::Reply(cmd.session_group, cmd.reply_frame);
        self.send_cmd_to_listener(listener_id, cmd)
    }
}
