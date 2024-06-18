// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use std::time::Instant;

use stdext::function_name;

use crate::session::Session;
use crate::session::status::Status;

impl Session {
    pub async fn run_loop(mut self) {
        let _connect_timeout = Instant::now();
        self.status = Status::Connected;
        let mut listener_receiver = self.listener_receiver.take().unwrap();

        while self.status != Status::Disconnected {
            tokio::select! {
                Some(frames) = self.read_frames() => {
                    self.frames_read.push_back(frames.len());
                    log::debug!("{} frames read: {}", function_name!(), frames.len());
                    if let Err(err) = self.handle_client_frames(frames).await {
                        log::warn!("fuck err: {err:?}");
                    }
                }
                Some(cmd) = listener_receiver.recv() => {
                    if let Err(err) = self.handle_listener_cmd(cmd).await {
                        log::error!("Failed to handle server packet: {:?}", err);
                    }
                    continue;
                },
            };
        }
        if let Err(err) = self.send_disconnect_to_listener().await {
            log::warn!("Failed to send disconnect info to listener, err: {err:?}");
        }

        log::info!("Session {} exit main loop", self.id);
        // Now session object goes out of scope and stream is dropped.
        if let Err(err) = self.stream.flush().await {
            log::warn!("Failed to flush stream, err: {err:?}");
        }
    }
}
