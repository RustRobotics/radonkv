// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use std::net::ToSocketAddrs;

use serde::Deserialize;

use crate::error::{Error, ErrorKind};

#[derive(Debug, Clone, Deserialize)]
pub struct Listener {
    #[serde(default = "Listener::default_bind_device")]
    bind_device: String,

    /// Binding address, including domain name and port.
    ///
    /// Default is `0.0.0.0:6379`.
    #[serde(default = "Listener::default_address")]
    address: String,
}

impl Default for Listener {
    fn default() -> Self {
        Self {
            bind_device: Self::default_bind_device(),
            address: Self::default_address(),
        }
    }
}

impl Listener {
    #[must_use]
    #[inline]
    pub fn bind_device(&self) -> &str {
        &self.bind_device
    }

    #[must_use]
    #[inline]
    pub fn address(&self) -> &str {
        &self.address
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

    #[must_use]
    #[inline]
    pub fn default_listeners() -> Vec<Self> {
        vec![Self::default()]
    }

    #[must_use]
    #[inline]
    pub const fn default_bind_device() -> String {
        String::new()
    }

    #[must_use]
    #[inline]
    pub fn default_address() -> String {
        "0.0.0.0:6379".to_owned()
    }
}