// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use tokio::sync::mpsc::Receiver;

use crate::commands::ListenerToDispatcherCmd;

mod listener;

#[derive(Debug)]
pub struct Dispatcher {
    listener_receiver: Receiver<ListenerToDispatcherCmd>,
}

impl Dispatcher {
    pub fn new(listener_receiver: Receiver<ListenerToDispatcherCmd>) -> Self {
        Self { listener_receiver }
    }

    pub async fn run_loop(&mut self) -> ! {
        loop {
            tokio::select! {
                Some(cmd) = self.listener_receiver.recv() => {
                    self.handle_listener_cmd(cmd).await;
                }
            }
        }
    }
}