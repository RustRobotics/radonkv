// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use bytes::BytesMut;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::commands::{ListenerToSessionCmd, SessionToListenerCmd};
use crate::listener::stream::Stream;
use crate::listener::types::SessionId;
use crate::session::config::SessionConfig;
use crate::session::status::Status;

pub mod config;
mod frame;
mod listener;
mod run;
mod status;

const BUF_SIZE: usize = 4096;

pub struct Session {
    id: SessionId,
    config: SessionConfig,

    status: Status,
    stream: Stream,
    buffer: BytesMut,

    listener_sender: Sender<SessionToListenerCmd>,
    listener_receiver: Option<Receiver<ListenerToSessionCmd>>,
}

impl Session {
    #[must_use]
    #[inline]
    pub fn new(
        id: SessionId,
        config: SessionConfig,
        stream: Stream,
        listener_sender: Sender<SessionToListenerCmd>,
        listener_receiver: Receiver<ListenerToSessionCmd>,
    ) -> Self {
        Self {
            id,
            config,

            status: Status::Invalid,
            stream,
            buffer: BytesMut::with_capacity(BUF_SIZE),

            listener_sender,
            listener_receiver: Some(listener_receiver),
        }
    }
}
