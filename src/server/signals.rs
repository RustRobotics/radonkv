// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::server::Server;

impl Server {
    // TODO(Shaohua): Reload config and send messages to other modules.
    pub fn reload_config(&mut self) {
        todo!();
    }

    pub fn terminate_server(&mut self) {
        self.running = false;
    }

    pub fn quit_server(&mut self) {
        self.running = false;
    }
}
