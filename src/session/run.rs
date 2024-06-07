// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use std::time::Instant;

use crate::commands::SessionToListenerCmd;
use crate::session::status::Status;
use crate::session::Session;

impl Session {
    pub async fn run_loop(mut self) {
        let _connect_timeout = Instant::now();

        loop {
            if self.status == Status::Disconnected {
                log::info!("session status is Disconnected");
                break;
            }

            tokio::select! {
                Ok(n_recv) = self.stream.read_buf(&mut self.buffer) => {
                    if n_recv > 0 {
                        if let Err(err) = self.handle_client_frame().await {
                            log::error!("handle_client_frame() failed: {:?}", err);
                        }
                    } else {
                        log::info!("session: Empty packet received, disconnect client, {}", self.id);
                        if let Err(err) = self.send_disconnect().await {
                            log::error!("session: Failed to send disconnect packet: {:?}", err);
                        }
                        break;
                    }
                }
                Some(cmd) = self.listener_receiver.recv() => {
                    if let Err(err) = self.handle_listener_cmd(cmd).await {
                        log::error!("Failed to handle server packet: {:?}", err);
                    }
                },
            }
        }

        if let Err(err) = self
            .listener_sender
            .send(SessionToListenerCmd::Disconnect(self.id))
            .await
        {
            log::error!(
                "Failed to send disconnect cmd to listener, id: {}, err: {:?}",
                self.id,
                err
            );
        }

        log::info!("Session {} exit main loop", self.id);
        // Now session object goes out of scope and stream is dropped.
    }
}
