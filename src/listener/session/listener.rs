// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::error::Error;
use crate::listener::commands::ListenerToSessionCmd;
use crate::listener::session::Session;

impl Session {
    pub(super) async fn handle_listener_cmd(&mut self, _cmd: ListenerToSessionCmd) -> Result<(), Error> {
        todo!()
    }
}