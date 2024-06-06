// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use std::fmt;

use tokio::net::TcpListener;

/// Each listener binds to a specific port and protocol.
pub enum SocketListener {
    Tcp(TcpListener),
}

impl fmt::Debug for SocketListener {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            Self::Tcp(..) => "Mqtt",
        };
        write!(f, "{msg}")
    }
}
