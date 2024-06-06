// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use std::time::Instant;

use tokio::io::AsyncReadExt;

use crate::listener::commands::SessionToListenerCmd;
use crate::listener::session::Session;
use crate::listener::session::status::Status;

impl Session {
    pub async fn run_loop(mut self) {
        let connect_timeout = Instant::now();

        loop {
            if self.status == Status::Disconnected {
                log::info!("session status is Disconnected");
                break;
            }

            tokio::select! {
                Ok(n_recv) = self.stream.read_buf(&mut self.buffer) => {
                    log::info!("n_recv: {}", n_recv);
                    if n_recv > 0 {
                        if let Err(err) = self.handle_client_packet().await {
                            log::error!("handle_client_packet() failed: {:?}", err);
                            break;
                        }
                        //buf.clear();
                    } else {
                        log::info!("session: Empty packet received, disconnect client, {}", self.id);
                        if let Err(err) = self.send_disconnect().await {
                            log::error!("session: Failed to send disconnect packet: {:?}", err);
                        }
                        break;
                    }
                }
                Some(cmd) = self.receiver.recv() => {
                    if let Err(err) = self.handle_listener_cmd(cmd).await {
                        log::error!("Failed to handle server packet: {:?}", err);
                    }
                },
            }
        }

        if let Err(err) = self
            .sender
            .send(SessionToListenerCmd::Disconnect(self.id))
            .await
        {
            log::error!(
                "Failed to send disconnect cmd to server, id: {}, err: {:?}",
                self.id,
                err
            );
        }

        log::info!("Session {} exit main loop", self.id);

        // Now session object goes out of scope and stream is dropped.
    }
}