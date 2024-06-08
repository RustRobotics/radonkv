// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use std::collections::HashMap;

use stdext::function_name;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::commands::{
    DispatcherToListenerCmd, ListenerToDispatcherCmd, ListenerToSessionCmd, SessionToListenerCmd,
};
use crate::config;
use crate::config::Protocol;
use crate::error::Error;
use crate::listener::socket::new_tcp_listener;
use crate::listener::socket_listener::SocketListener;
use crate::listener::stream::Stream;
use crate::listener::types::{ListenerId, SessionId};
use crate::session::config::SessionConfig;
use crate::session::Session;

mod dispatcher;
mod run;
mod session;
mod socket;
mod socket_listener;
pub mod stream;
pub mod types;

#[derive(Debug)]
pub struct Listener {
    id: ListenerId,
    config: config::Listener,
    current_session_id: SessionId,
    socket_listener: SocketListener,

    session_senders: HashMap<SessionId, Sender<ListenerToSessionCmd>>,
    session_sender: Sender<SessionToListenerCmd>,
    session_receiver: Option<Receiver<SessionToListenerCmd>>,

    dispatcher_sender: Sender<ListenerToDispatcherCmd>,
    dispatcher_receiver: Option<Receiver<DispatcherToListenerCmd>>,
}

const CHANNEL_CAPACITY: usize = 1024 * 8;

impl Listener {
    #[must_use]
    #[inline]
    fn next_session_id(&mut self) -> SessionId {
        self.current_session_id += 1;
        self.current_session_id
    }

    fn new_connection(&mut self, stream: Stream) {
        let (sender, receiver) = mpsc::channel(CHANNEL_CAPACITY);
        let session_id = self.next_session_id();
        log::info!(
            "Got new connection in listener {}, session id: {session_id}",
            self.id
        );
        self.session_senders.insert(session_id, sender);
        let session_config = SessionConfig::new(self.config.keepalive());
        let session = Session::new(
            session_id,
            session_config,
            stream,
            self.session_sender.clone(),
            receiver,
        );
        tokio::spawn(session.run_loop());
    }

    /// Bind to specific socket address.
    pub(super) async fn bind(
        id: ListenerId,
        listener_config: config::Listener,
        dispatcher_sender: Sender<ListenerToDispatcherCmd>,
        dispatcher_receiver: Receiver<DispatcherToListenerCmd>,
    ) -> Result<Self, Error> {
        let device = listener_config.bind_device().to_owned();
        let address = listener_config.address().to_owned();
        log::info!("{} device: {device}, address: {address}", function_name!());

        let (session_sender, session_receiver) = mpsc::channel(CHANNEL_CAPACITY);
        let protocol = listener_config.protocol();

        let new_listener = |socket_listener: SocketListener| {
            Ok(Self {
                id,
                config: listener_config,
                current_session_id: 0,
                socket_listener,

                session_senders: HashMap::new(),
                session_sender,
                session_receiver: Some(session_receiver),

                dispatcher_sender,
                dispatcher_receiver: Some(dispatcher_receiver),
            })
        };

        match protocol {
            Protocol::Tcp => {
                log::info!("bind to tcp://{}", address);
                let listener = new_tcp_listener(&address, &device).await?;
                new_listener(SocketListener::Tcp(listener))
            }
            Protocol::Tls => {
                unimplemented!()
            }
            Protocol::Ws => {
                unimplemented!()
            }
            Protocol::Wss => {
                unimplemented!()
            }
            Protocol::Quic => {
                unimplemented!()
            }
            Protocol::Uds => {
                unimplemented!()
            }
        }
    }

    pub(super) async fn accept(&mut self) -> Result<Stream, Error> {
        match &mut self.socket_listener {
            SocketListener::Tcp(tcp_listener) => {
                let (tcp_stream, _address) = tcp_listener.accept().await?;
                Ok(Stream::Tcp(tcp_stream))
            }
        }
    }
}
