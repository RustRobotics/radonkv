// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cluster::Cluster;

impl Cluster {
    pub async fn run_loop(&mut self) -> ! {
        loop {
            if let Some(cmd) = self.dispatcher_receiver.recv().await {
                if let Err(err) = self.handle_dispatcher_cmd(cmd).await {
                    log::warn!("[cluster] Failed to handle dispatcher cmd, err: {err:?}");
                }
            }
        }
    }
}
