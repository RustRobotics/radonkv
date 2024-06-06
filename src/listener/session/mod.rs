// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use bytes::BytesMut;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::listener::commands::{ListenerToSessionCmd, SessionToListenerCmd};
use crate::listener::session::config::SessionConfig;
use crate::listener::session::status::Status;
use crate::listener::stream::Stream;
use crate::listener::types::SessionId;

pub mod config;
mod run;
mod status;

const BUF_SIZE: usize = 4096;

pub struct Session {
    id: SessionId,
    config: SessionConfig,

    status: Status,
    stream: Stream,
    buffer: BytesMut,

    sender: Sender<SessionToListenerCmd>,
    receiver: Receiver<ListenerToSessionCmd>,
}

impl Session {
    #[must_use]
    #[inline]
    pub const fn new(id: SessionId, config: SessionConfig, stream: Stream,
                     sender: Sender<SessionToListenerCmd>,
                     receiver: Receiver<ListenerToSessionCmd>,
    ) -> Self {
        Self {
            id,
            config,

            status: Status::Invalid,
            stream,
            buffer: BytesMut::with_capacity(BUF_SIZE),

            sender,
            receiver,
        }
    }
}