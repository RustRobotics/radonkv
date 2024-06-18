// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::dispatcher::Dispatcher;

impl Dispatcher {
    pub async fn run_loop(&mut self) -> ! {
        loop {
            tokio::select! {
                Some(cmd) = self.listener_receiver.recv() => {
                    if let Err(err) = self.handle_listener_cmd(cmd).await {
                        log::warn!("[dispatcher] Failed to handle listener cmd, got err: {err:?}");
                    }
                }
                Some(cmd) = self.mem_receiver.recv() => {
                    if let Err(err) = self.handle_mem_cmd(cmd).await {
                        log::warn!("[dispatcher] Failed to handle mem cmd, got err: {err:?}");
                    }
                }
                Some(cmd) = self.cluster_receiver.recv() => {
                     if let Err(err) = self.handle_cluster_cmd(cmd).await {
                        log::warn!("[dispatcher] Failed to handle cluster cmd, got err: {err:?}");
                    }
                }
                Some(cmd) = self.server_receiver.recv() => {
                     if let Err(err) = self.handle_server_cmd(cmd).await {
                        log::warn!("[dispatcher] Failed to handle server cmd, got err: {err:?}");
                    }
                }
                Some(cmd) = self.storage_receiver.recv() => {
                      if let Err(err) = self.handle_storage_cmd(cmd).await {
                        log::warn!("[dispatcher] Failed to handle storage cmd, got err: {err:?}");
                    }
                }
            }
        }
    }
}
