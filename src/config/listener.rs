// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use std::net::ToSocketAddrs;

use serde::Deserialize;

use crate::error::{Error, ErrorKind};

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Deserialize)]
pub enum Protocol {
    /// Raw redis wire protocol in TCP.
    #[serde(alias = "tcp")]
    #[default]
    Tcp,

    /// TCP with TLS encryption.
    #[serde(alias = "tls")]
    Tls,

    /// Websocket.
    #[serde(alias = "ws")]
    Ws,

    /// Secure Websocket.
    #[serde(alias = "wss")]
    Wss,

    /// QUIC protocol.
    #[serde(alias = "quic")]
    Quic,

    /// Redis wire protocol through unix domain socket.
    #[serde(alias = "uds")]
    Uds,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Listener {
    /// Binding address, including domain name and port.
    ///
    /// Default is `0.0.0.0:6379`.
    #[serde(default = "Listener::default_address")]
    address: String,

    #[serde(default = "Listener::default_bind_device")]
    bind_device: String,

    /// Connection keepalive timeout in seconds.
    ///
    /// Default is 60s.
    #[serde(default = "Listener::default_keepalive")]
    keepalive: u16,

    /// The maximum number of client connections to this listener allowed.
    ///
    /// Default is 0, which means unlimited.
    #[serde(default = "Listener::default_max_connections")]
    max_connections: usize,

    /// Binding protocol.
    ///
    /// Default is TCP.
    #[serde(default = "Listener::default_protocol")]
    protocol: Protocol,
}

impl Default for Listener {
    fn default() -> Self {
        Self {
            address: Self::default_address(),
            bind_device: Self::default_bind_device(),
            keepalive: Self::default_keepalive(),
            max_connections: Self::default_max_connections(),
            protocol: Self::default_protocol(),
        }
    }
}

impl Listener {
    #[must_use]
    #[inline]
    pub fn address(&self) -> &str {
        &self.address
    }

    #[must_use]
    #[inline]
    pub fn bind_device(&self) -> &str {
        &self.bind_device
    }

    #[must_use]
    #[inline]
    pub const fn keepalive(&self) -> u16 {
        self.keepalive
    }

    #[must_use]
    #[inline]
    pub const fn max_connections(&self) -> usize {
        self.max_connections
    }

    #[must_use]
    #[inline]
    pub const fn protocol(&self) -> Protocol {
        self.protocol
    }

    #[must_use]
    #[inline]
    pub fn default_listeners() -> Vec<Self> {
        vec![Self::default()]
    }

    #[must_use]
    #[inline]
    pub fn default_address() -> String {
        "0.0.0.0:6379".to_owned()
    }

    #[must_use]
    #[inline]
    pub const fn default_bind_device() -> String {
        String::new()
    }

    #[must_use]
    #[inline]
    pub const fn default_keepalive() -> u16 {
        60
    }

    #[must_use]
    #[inline]
    pub const fn default_max_connections() -> usize {
        0
    }

    #[must_use]
    #[inline]
    pub const fn default_protocol() -> Protocol {
        Protocol::Tcp
    }

    pub fn validate(&self) -> Result<(), Error> {
        if let Err(err) = self.address.to_socket_addrs() {
            Err(Error::from_string(
                ErrorKind::ConfigError,
                format!("Invalid socket address: {}, err: {err:?}", &self.address),
            ))
        } else {
            Ok(())
        }
    }
}
