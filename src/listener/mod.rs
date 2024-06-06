// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::config;
use crate::listener::types::{ListenerId, SessionId};

mod types;
mod session;
mod stream;
mod socket;
mod run;
mod init;
mod socket_unix;

#[derive(Debug)]
pub struct Listener {
    id: ListenerId,
    config: config::Listener,
    current_session_id: SessionId,
}
