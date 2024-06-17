// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::reply_frame::ReplyFrame;
use crate::cmd::server_mgmt::ServerManagementCommand;
use crate::commands::{DispatcherToServerCmd, ServerToDispatcherCmd};
use crate::error::{Error, ErrorKind};
use crate::server::commands::time;
use crate::server::Server;

impl Server {
    pub async fn handle_dispatcher_cmd(&mut self, cmd: DispatcherToServerCmd) -> Result<(), Error> {
        let session_group = cmd.session_group;

        let reply_frame: ReplyFrame = match cmd.command {
            ServerManagementCommand::Shutdown => {
                self.quit_server();
                return Ok(());
            }
            ServerManagementCommand::Time => time::time(),
        };

        if let Some(sender) = &self.dispatcher_sender {
            let msg = ServerToDispatcherCmd {
                session_group,
                reply_frame,
            };
            sender.send(msg).await?;
            Ok(())
        } else {
            Err(Error::from_string(
                ErrorKind::InternalError,
                "Failed to get dispatcher sender in server module.".to_owned(),
            ))
        }
    }
}
