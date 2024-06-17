// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::commands::DispatcherToServerCmd;
use crate::error::Error;
use crate::server::Server;

impl Server {
    pub async fn handle_dispatcher_cmd(
        &mut self,
        _command: DispatcherToServerCmd,
    ) -> Result<(), Error> {
        todo!()
    }
}
