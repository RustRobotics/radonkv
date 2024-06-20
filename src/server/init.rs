// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use std::collections::HashMap;

use stdext::function_name;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;

use crate::cluster::Cluster;
use crate::dispatcher::Dispatcher;
use crate::error::Error;
use crate::listener::types::ListenerId;
use crate::listener::Listener;
use crate::mem::Mem;
use crate::server::Server;
use crate::storage::Storage;

// 64k
const CHANNEL_CAPACITY: usize = 1024 * 64;

impl Server {
    pub(crate) async fn init_modules(&mut self, runtime: &Runtime) -> Result<(), Error> {
        log::info!("{}", function_name!());

        let (listeners_to_dispatcher_sender, listeners_to_dispatcher_receiver) =
            mpsc::channel(CHANNEL_CAPACITY);
        let mut dispatcher_to_listener_senders = HashMap::new();
        let mut listeners_info = Vec::new();

        // Listeners module.
        let mut listener_objs = Vec::new();
        for (listener_id, listener_config) in self.config.listeners().iter().enumerate() {
            let listener_id = ListenerId::try_from(listener_id).unwrap();
            listeners_info.push((listener_id, listener_config.address().to_owned()));
            let (dispatcher_to_listener_sender, dispatcher_to_listener_receiver) =
                mpsc::channel(CHANNEL_CAPACITY);
            dispatcher_to_listener_senders.insert(listener_id, dispatcher_to_listener_sender);

            let listener = Listener::bind(
                listener_id,
                listener_config.clone(),
                // dispatcher module
                listeners_to_dispatcher_sender.clone(),
                dispatcher_to_listener_receiver,
            )
            .await
            .unwrap_or_else(|_| panic!("Failed to listen at {:?}", &listeners_info.last()));
            listener_objs.push(listener);
        }

        for mut listener in listener_objs {
            let _listener_handle = runtime.spawn(async move {
                listener.run_loop().await;
            });
        }

        // Mem module
        let (mem_to_dispatcher_sender, mem_to_dispatcher_receiver) =
            mpsc::channel(CHANNEL_CAPACITY);
        let (dispatcher_to_mem_sender, dispatcher_to_mem_receiver) =
            mpsc::channel(CHANNEL_CAPACITY);
        let mut mem = Mem::new(mem_to_dispatcher_sender, dispatcher_to_mem_receiver);
        let _mem_handle = runtime.spawn(async move {
            mem.run_loop().await;
        });

        // Cluster module
        let (cluster_to_dispatcher_sender, cluster_to_dispatcher_receiver) =
            mpsc::channel(CHANNEL_CAPACITY);
        let (dispatcher_to_cluster_sender, dispatcher_to_cluster_receiver) =
            mpsc::channel(CHANNEL_CAPACITY);
        let mut cluster =
            Cluster::new(cluster_to_dispatcher_sender, dispatcher_to_cluster_receiver);
        let _cluster_handle = runtime.spawn(async move {
            cluster.run_loop().await;
        });

        // Storage module
        let (storage_to_dispatcher_sender, storage_to_dispatcher_receiver) =
            mpsc::channel(CHANNEL_CAPACITY);
        let (dispatcher_to_storage_sender, dispatcher_to_storage_receiver) =
            mpsc::channel(CHANNEL_CAPACITY);
        let mut storage =
            Storage::new(storage_to_dispatcher_sender, dispatcher_to_storage_receiver);
        let _storage_handle = runtime.spawn(async move {
            storage.run_loop().await;
        });

        // self module
        let (server_to_dispatcher_sender, server_to_dispatcher_receiver) =
            mpsc::channel(CHANNEL_CAPACITY);
        let (dispatcher_to_server_sender, dispatcher_to_server_receiver) =
            mpsc::channel(CHANNEL_CAPACITY);

        self.dispatcher_sender = Some(server_to_dispatcher_sender);
        self.dispatcher_receiver = Some(dispatcher_to_server_receiver);

        // Dispatcher module
        let mut dispatcher = Dispatcher::new(
            // listeners module
            dispatcher_to_listener_senders,
            listeners_to_dispatcher_receiver,
            // mem module
            dispatcher_to_mem_sender,
            mem_to_dispatcher_receiver,
            // cluster module
            dispatcher_to_cluster_sender,
            cluster_to_dispatcher_receiver,
            // storage module
            dispatcher_to_storage_sender,
            storage_to_dispatcher_receiver,
            // server module,
            dispatcher_to_server_sender,
            server_to_dispatcher_receiver,
        );
        let _dispatcher_handle = runtime.spawn(async move {
            dispatcher.run_loop().await;
        });

        Ok(())
    }
}
