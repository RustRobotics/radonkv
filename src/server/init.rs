// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use stdext::function_name;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;

use crate::dispatcher::Dispatcher;
use crate::error::Error;
use crate::listener::Listener;
use crate::listener::types::ListenerId;
use crate::server::Server;
use crate::storage::Storage;

impl Server {
    pub(crate) async fn init_modules(&mut self, runtime: &Runtime) -> Result<(), Error> {
        log::info!("{}", function_name!());

        const CHANNEL_CAPACITY: usize = 16;

        let (listeners_to_dispatcher_sender, listeners_to_dispatcher_receiver) =
            mpsc::channel(CHANNEL_CAPACITY);
        let mut dispatcher_to_listener_senders = Vec::new();
        let mut handles = Vec::new();
        let mut listeners_info = Vec::new();

        // Listeners module.
        let mut listener_objs = Vec::new();
        for (listener_id, listener_config) in self.config.listeners().iter().enumerate() {
            let listener_id = listener_id as ListenerId;
            listeners_info.push((listener_id, listener_config.address().to_owned()));
            let (dispatcher_to_listener_sender, dispatcher_to_listener_receiver) =
                mpsc::channel(CHANNEL_CAPACITY);
            dispatcher_to_listener_senders.push((listener_id, dispatcher_to_listener_sender));

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
            let handle = runtime.spawn(async move {
                listener.run_loop().await;
            });
            handles.push(handle);
        }

        // Storage module
        let (storage_to_dispatcher_sender, storage_to_dispatcher_receiver) =
            mpsc::channel(CHANNEL_CAPACITY);
        let (dispatcher_to_storage_sender, dispatcher_to_storage_receiver) = mpsc::channel(CHANNEL_CAPACITY);
        let mut storage = Storage::new(
            storage_to_dispatcher_sender,
            dispatcher_to_storage_receiver,
        );
        let storage_handle = runtime.spawn(async move {
            storage.run_loop().await;
        });
        handles.push(storage_handle);

        // Dispatcher module
        let mut dispatcher = Dispatcher::new(
            // listeners module
            dispatcher_to_listener_senders,
            listeners_to_dispatcher_receiver,
            // storage module
            dispatcher_to_storage_sender,
            storage_to_dispatcher_receiver,
        );
        let dispatcher_handle = runtime.spawn(async move {
            dispatcher.run_loop().await;
        });
        handles.push(dispatcher_handle);

        Ok(())
    }
}