// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::listener::Listener;

impl Listener {
    pub async fn run_loop(&mut self) -> ! {
        loop {
            tokio::select! {
                Ok(stream) = self.connect() => {
                    self.new_connection(stream).await;
                }
            }
        }
    }
}