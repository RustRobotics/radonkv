// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use tokio::sync::mpsc::{Receiver, Sender};

use crate::commands::{ClusterToDispatcherCmd, DispatcherToClusterCmd};

mod commands;
mod dispatcher;
pub mod run;

#[derive(Debug)]
pub struct Cluster {
    dispatcher_sender: Sender<ClusterToDispatcherCmd>,
    dispatcher_receiver: Receiver<DispatcherToClusterCmd>,
}

impl Cluster {
    #[must_use]
    #[inline]
    pub const fn new(
        dispatcher_sender: Sender<ClusterToDispatcherCmd>,
        dispatcher_receiver: Receiver<DispatcherToClusterCmd>,
    ) -> Self {
        Self {
            dispatcher_sender,
            dispatcher_receiver,
        }
    }
}
