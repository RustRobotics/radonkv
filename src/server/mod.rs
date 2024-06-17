// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use tokio::sync::mpsc::{Receiver, Sender};

use crate::commands::{DispatcherToServerCmd, ServerToDispatcherCmd};
use crate::config::Config;

mod commands;
mod dispatcher;
mod init;
pub mod run;
mod signals;

pub struct Server {
    pub config: Config,
    running: bool,

    dispatcher_sender: Option<Sender<ServerToDispatcherCmd>>,
    dispatcher_receiver: Option<Receiver<DispatcherToServerCmd>>,
}

impl Server {
    #[must_use]
    pub const fn new(config: Config) -> Self {
        Self {
            config,
            running: false,

            dispatcher_sender: None,
            dispatcher_receiver: None,
        }
    }
}
