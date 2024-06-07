// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use tokio::sync::mpsc::{Receiver, Sender};

use crate::commands::{DispatcherToStorageCmd, StorageToDispatcherCmd};

mod dispatcher;
mod run;

#[derive(Debug)]
pub struct Storage {
    dispatcher_sender: Sender<StorageToDispatcherCmd>,
    dispatcher_receiver: Receiver<DispatcherToStorageCmd>,
}

impl Storage {
    #[must_use]
    pub const fn new(
        dispatcher_sender: Sender<StorageToDispatcherCmd>,
        dispatcher_receiver: Receiver<DispatcherToStorageCmd>,
    ) -> Self {
        Self {
            dispatcher_sender,
            dispatcher_receiver,
        }
    }
}
