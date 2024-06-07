// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use tokio::runtime::Runtime;

use crate::config::Config;
use crate::error::Error;
use crate::log::init_log;
use crate::server::context::ServerContext;

pub fn handle_cmdline() -> Result<(), Error> {
    let config = Config::default();
    init_log(&config.log())?;

    let mut server = ServerContext::new(config);
    // TODO(Shaohua): Check signal options

    let runtime = Runtime::new()?;
    server.run_loop(&runtime)
}

pub fn run_server_with_config(config: Config) -> Result<(), Error> {
    init_log(&config.log())?;
    let mut server = ServerContext::new(config);
    let runtime = Runtime::new()?;
    server.run_loop(&runtime)
}
