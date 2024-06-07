// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::commands::SessionToListenerCmd;
use crate::error::Error;
use crate::listener::Listener;

impl Listener {
    pub(super) async fn handle_session_cmd(&mut self, _cmd: SessionToListenerCmd) -> Result<(), Error> {
        todo!()
    }
}