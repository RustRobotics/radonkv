// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use std::collections::HashMap;

use tokio::sync::mpsc::{Receiver, Sender};

use crate::commands::{DispatcherToListenerCmd, ListenerToDispatcherCmd};
use crate::listener::types::ListenerId;

mod listener;

#[derive(Debug)]
pub struct Dispatcher {
    listener_senders: HashMap<ListenerId, Sender<DispatcherToListenerCmd>>,
    listener_receiver: Receiver<ListenerToDispatcherCmd>,
}

impl Dispatcher {
    pub fn new(
        listener_senders: Vec<(ListenerId, Sender<DispatcherToListenerCmd>)>,
        listener_receiver: Receiver<ListenerToDispatcherCmd>,
    ) -> Self {
        Self {
            listener_senders: listener_senders.into_iter().collect(),
            listener_receiver,
        }
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