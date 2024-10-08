// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use std::collections::VecDeque;

use bytes::BytesMut;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::cmd::reply_frame::ReplyFrame;
use crate::commands::{ListenerToSessionCmd, SessionToListenerCmd};
use crate::listener::stream::Stream;
use crate::listener::types::SessionId;
use crate::session::config::SessionConfig;
use crate::session::status::Status;

mod commands;
pub mod config;
mod conn;
mod frame;
mod listener;
mod run;
mod status;

const BUF_SIZE: usize = 4096;

pub struct Session {
    id: SessionId,
    name: Option<String>,
    config: SessionConfig,

    status: Status,
    stream: Stream,
    buffer: BytesMut,
    frames_read: VecDeque<usize>,
    pending_frames: Vec<ReplyFrame>,

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
            name: None,
            config,

            status: Status::Invalid,
            stream,
            buffer: BytesMut::with_capacity(BUF_SIZE),
            frames_read: VecDeque::new(),
            pending_frames: Vec::new(),

            listener_sender,
            listener_receiver: Some(listener_receiver),
        }
    }
}
