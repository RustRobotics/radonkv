// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::storage_mgmt::StorageManagementCommand;
use crate::commands::{DispatcherToStorageCmd, StorageToDispatcherCmd};
use crate::error::Error;
use crate::storage::commands::save;
use crate::storage::Storage;

impl Storage {
    #[allow(clippy::unused_async)]
    pub(super) async fn handle_dispatcher_cmd(
        &mut self,
        cmd: DispatcherToStorageCmd,
    ) -> Result<(), Error> {
        let session_group = cmd.session_group;

        let reply_frame = match cmd.command {
            StorageManagementCommand::Save => save::save(),
        };

        let msg = StorageToDispatcherCmd {
            session_group,
            reply_frame,
        };
        self.dispatcher_sender.send(msg).await?;
        Ok(())
    }
}
