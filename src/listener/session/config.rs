// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

#[derive(Debug, Clone)]
pub struct SessionConfig {
    keepalive: u16,
}

impl SessionConfig {
    #[must_use]
    #[inline]
    pub const fn new(keepalive: u16) -> Self {
        Self {
            keepalive,
        }
    }

    #[must_use]
    #[inline]
    pub const fn keepalive(&self) -> u16 {
        self.keepalive
    }
}