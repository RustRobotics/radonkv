// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use tokio::sync::mpsc::{Receiver, Sender};

use crate::commands::{DispatcherToMemCmd, MemToDispatcherCmd};

mod set;
mod zset;
mod hash;
mod hyperloglog;
mod list;
mod bitmap;
mod geo;
mod stream;
mod pub_sub;
mod bloom_filter;
mod cuckoo_filter;
mod count_min_sketch;
mod json;
mod top_k;
mod time_series;
mod string;
mod auto_suggest;
mod run;
mod dispatcher;

#[derive(Debug)]
pub struct Mem {
    dispatcher_sender: Sender<MemToDispatcherCmd>,
    dispatcher_receiver: Receiver<DispatcherToMemCmd>,
}

impl Mem {
    #[must_use]
    #[inline]
    pub const fn new(
        dispatcher_sender: Sender<MemToDispatcherCmd>,
        dispatcher_receiver: Receiver<DispatcherToMemCmd>,
    ) -> Self {
        Self {
            dispatcher_sender,
            dispatcher_receiver,
        }
    }
}