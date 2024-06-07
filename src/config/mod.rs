// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use serde::Deserialize;

pub use listener::{Listener, Protocol};
pub use log::{Log, LogLevel};

use crate::error::Error;

mod listener;
mod log;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(default = "Listener::default_listeners")]
    listeners: Vec<Listener>,

    log: Log,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            listeners: Listener::default_listeners(),
            log: Log::default(),
        }
    }
}

impl Config {
    #[must_use]
    #[inline]
    pub fn listeners(&self) -> &[Listener] {
        &self.listeners
    }

    #[must_use]
    #[inline]
    pub const fn log(&self) -> &Log {
        &self.log
    }

    pub fn validate(&self) -> Result<(), Error> {
        for listener in &self.listeners {
            listener.validate()?;
        }
        Ok(())
    }
}
