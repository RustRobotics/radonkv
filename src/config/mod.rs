// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use serde::Deserialize;

pub use listener::{Listener, Protocol};

use crate::error::Error;

mod listener;

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Config {
    #[serde(default = "Listener::default_listeners")]
    listeners: Vec<Listener>,
}

impl Config {
    #[must_use]
    #[inline]
    pub fn listeners(&self) -> &[Listener] {
        &self.listeners
    }

    pub fn validate(&self) -> Result<(), Error> {
        for listener in &self.listeners {
            listener.validate()?;
        }
        Ok(())
    }
}