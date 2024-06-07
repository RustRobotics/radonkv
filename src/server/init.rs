// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use stdext::function_name;
use tokio::sync::mpsc;

use crate::dispatcher::Dispatcher;
use crate::error::Error;
use crate::listener::Listener;
use crate::listener::types::ListenerId;
use crate::server::context::ServerContext;

impl ServerContext {
    pub(crate) async fn init_modules(&mut self) -> Result<(), Error> {
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
            let handle = tokio::spawn(async move {
                listener.run_loop().await;
            });
            handles.push(handle);
        }

        // Dispatcher module
        let mut dispatcher = Dispatcher::new(
            // listeners module
            dispatcher_to_listener_senders,
            listeners_to_dispatcher_receiver,
        );
        let dispatcher_handle = tokio::spawn(async move {
            dispatcher.run_loop().await;
        });
        handles.push(dispatcher_handle);


        Ok(())
    }
}