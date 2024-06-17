// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use stdext::function_name;
use tokio::runtime::Runtime;
use tokio::signal::unix::{signal, SignalKind};

use crate::config::Config;
use crate::error::Error;
use crate::log::init_log;
use crate::server::Server;

pub fn handle_cmdline() -> Result<(), Error> {
    let config = Config::default();
    init_log(config.log())?;
    log::info!("config: {config:#?}");

    let mut server = Server::new(config);
    // TODO(Shaohua): Check signal options

    let runtime = Runtime::new()?;
    server.run_loop(&runtime)
}

pub fn run_server_with_config(config: Config) -> Result<(), Error> {
    init_log(config.log())?;
    log::info!("config: {config:#?}");
    let mut server = Server::new(config);
    let runtime = Runtime::new()?;
    server.run_loop(&runtime)
}

impl Server {
    pub fn run_loop(&mut self, runtime: &Runtime) -> Result<(), Error> {
        if let Err(err) = self.config.validate() {
            eprintln!("Failed to validate config file!");
            return Err(err);
        }

        runtime.block_on(async {
            self.init_modules(runtime).await?;
            self.run_inner_loop().await
        })
    }

    async fn run_inner_loop(&mut self) -> Result<(), Error> {
        log::info!("{}", function_name!());

        let mut sig_user1 = signal(SignalKind::user_defined1())?;
        let mut sig_term = signal(SignalKind::terminate())?;
        let mut sig_quit = signal(SignalKind::quit())?;
        let mut sig_interrupt = signal(SignalKind::interrupt())?;

        let mut dispatcher_receiver = self
            .dispatcher_receiver
            .take()
            .expect("dispatcher receiver is not set");

        loop {
            tokio::select! {
                Some(_signum) = sig_user1.recv() => {
                    log::info!("Reload config");
                    // TODO(Shaohua): Reload config and send messages to other modules.
                }
                Some(_signum) = sig_term.recv() => {
                    log::info!("Quit with SIGTERM");
                    break;
                }
                Some(_signum) = sig_quit.recv() => {
                    log::info!("Quit with SIGQUIT");
                    break;
                }
                Some(_signum) = sig_interrupt.recv() => {
                    log::info!("Quit with SIGINT");
                    break;
                }

                Some(cmd) = dispatcher_receiver.recv() => {
                     if let Err(err) = self.handle_dispatcher_cmd(cmd).await {
                        log::warn!("[dispatcher] Failed to handle dispatcher cmd, got err: {err:?}");
                    }
                }
            }
        }

        Ok(())
    }
}
