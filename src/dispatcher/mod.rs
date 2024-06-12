// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use std::collections::HashMap;

use tokio::sync::mpsc::{Receiver, Sender, UnboundedReceiver, UnboundedSender};

use crate::commands::{
    DispatcherToListenerCmd, DispatcherToMemCmd, DispatcherToStorageCmd, ListenerToDispatcherCmd,
    MemToDispatcherCmd, StorageToDispatcherCmd,
};
use crate::listener::types::ListenerId;

mod listener;
mod mem;
mod run;
mod storage;

#[derive(Debug)]
pub struct Dispatcher {
    listener_senders: HashMap<ListenerId, UnboundedSender<DispatcherToListenerCmd>>,
    listener_receiver: UnboundedReceiver<ListenerToDispatcherCmd>,

    mem_sender: UnboundedSender<DispatcherToMemCmd>,
    mem_receiver: UnboundedReceiver<MemToDispatcherCmd>,

    storage_sender: Sender<DispatcherToStorageCmd>,
    storage_receiver: Receiver<StorageToDispatcherCmd>,
}

impl Dispatcher {
    #[must_use]
    pub fn new(
        listener_senders: Vec<(ListenerId, UnboundedSender<DispatcherToListenerCmd>)>,
        listener_receiver: UnboundedReceiver<ListenerToDispatcherCmd>,
        mem_sender: UnboundedSender<DispatcherToMemCmd>,
        mem_receiver: UnboundedReceiver<MemToDispatcherCmd>,
        storage_sender: Sender<DispatcherToStorageCmd>,
        storage_receiver: Receiver<StorageToDispatcherCmd>,
    ) -> Self {
        Self {
            listener_senders: listener_senders.into_iter().collect(),
            listener_receiver,

            mem_sender,
            mem_receiver,

            storage_sender,
            storage_receiver,
        }
    }
}
