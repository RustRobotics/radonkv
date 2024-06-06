// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::config;
use crate::config::Protocol;
use crate::error::Error;
use crate::listener::Listener;
use crate::listener::types::ListenerId;

impl Listener {
    /// Bind to specific socket address.
    pub(super) async fn bind(id: ListenerId, listener_config: config::Listener) -> Result<Self, Error> {
        let device = listener_config.bind_device();
        let address = listener_config.address();

        let new_listener = |protocol: Protocol| {
            Self {
                id,
                config: listener_config,
                current_session_id: 0,
            }
        };

        match listener_config.protocol() {
            Protocol::Tcp => {
                log::info!("bind to tcp://{}", address);
                let listener = new_tcp_listener(address, device).await?;
                new_listener(Protocol::Tcp(listener))
            }
            Protocol::Tls => {
                unimplemented!()
            }
            Protocol::Ws => {
                unimplemented!()
            }
            Protocol::Wss => {
                unimplemented!()
            }
            Protocol::Quic => {
                unimplemented!()
            }
            Protocol::Uds => {
                unimplemented!()
            }
        }
    }
}