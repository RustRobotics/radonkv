// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::Command;
use crate::cmd::conn_mgmt::ConnectManagementCommand;
use crate::error::Error;
use crate::session::{commands, Session};

impl Session {
    pub(super) async fn handle_client_command(&mut self, command: Command) -> Result<(), Error> {
        if let Command::ConnManagement(cmd) = command {
            let reply_frame = match cmd {
                ConnectManagementCommand::GetId() => commands::get_id(self.id),
                ConnectManagementCommand::Echo(message) => commands::echo(message),
                ConnectManagementCommand::Ping(message) => commands::ping(message),
                ConnectManagementCommand::GetName() => commands::get_name(self.name.as_ref()),
                ConnectManagementCommand::SetName(new_name) => {
                    commands::set_name(&mut self.name, new_name)
                }
            };
            self.send_frame_to_client(reply_frame).await
        } else {
            unreachable!()
        }
    }
}
