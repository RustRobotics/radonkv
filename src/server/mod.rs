// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::config::Config;

mod init;
pub mod run;

pub struct Server {
    pub config: Config,
}

impl Server {
    #[must_use]
    pub const fn new(config: Config) -> Self {
        Self { config }
    }
}
