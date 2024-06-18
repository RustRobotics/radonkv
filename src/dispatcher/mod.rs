// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use std::collections::HashMap;

use tokio::sync::mpsc::{Receiver, Sender};

use crate::commands::{
    ClusterToDispatcherCmd, DispatcherToClusterCmd, DispatcherToListenerCmd, DispatcherToMemCmd,
    DispatcherToServerCmd, DispatcherToStorageCmd, ListenerToDispatcherCmd, MemToDispatcherCmd,
    ServerToDispatcherCmd, StorageToDispatcherCmd,
};
use crate::listener::types::ListenerId;

mod cluster;
mod listener;
mod mem;
mod run;
mod server;
mod storage;

#[derive(Debug)]
pub struct Dispatcher {
    listener_senders: HashMap<ListenerId, Sender<DispatcherToListenerCmd>>,
    listener_receiver: Receiver<ListenerToDispatcherCmd>,

    mem_sender: Sender<DispatcherToMemCmd>,
    mem_receiver: Receiver<MemToDispatcherCmd>,

    cluster_sender: Sender<DispatcherToClusterCmd>,
    cluster_receiver: Receiver<ClusterToDispatcherCmd>,

    storage_sender: Sender<DispatcherToStorageCmd>,
    storage_receiver: Receiver<StorageToDispatcherCmd>,

    server_sender: Sender<DispatcherToServerCmd>,
    server_receiver: Receiver<ServerToDispatcherCmd>,
}

impl Dispatcher {
    #[must_use]
    pub fn new(
        listener_senders: Vec<(ListenerId, Sender<DispatcherToListenerCmd>)>,
        listener_receiver: Receiver<ListenerToDispatcherCmd>,
        mem_sender: Sender<DispatcherToMemCmd>,
        mem_receiver: Receiver<MemToDispatcherCmd>,
        cluster_sender: Sender<DispatcherToClusterCmd>,
        cluster_receiver: Receiver<ClusterToDispatcherCmd>,
        storage_sender: Sender<DispatcherToStorageCmd>,
        storage_receiver: Receiver<StorageToDispatcherCmd>,
        server_sender: Sender<DispatcherToServerCmd>,
        server_receiver: Receiver<ServerToDispatcherCmd>,
    ) -> Self {
        Self {
            listener_senders: listener_senders.into_iter().collect(),
            listener_receiver,

            mem_sender,
            mem_receiver,

            cluster_sender,
            cluster_receiver,

            storage_sender,
            storage_receiver,

            server_sender,
            server_receiver,
        }
    }
}
