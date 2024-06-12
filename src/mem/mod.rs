// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use std::collections::HashMap;

use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::commands::{DispatcherToMemCmd, MemToDispatcherCmd};
use crate::mem::db::Db;

mod auto_suggest;
mod bitmap;
mod bloom_filter;
mod count_min_sketch;
mod cuckoo_filter;
mod db;
mod dispatcher;
mod generic;
mod geo;
mod hash;
mod hyperloglog;
mod json;
mod list;
mod pub_sub;
mod run;
mod set;
mod stream;
mod string;
mod time_series;
mod top_k;
mod util;
mod zset;

#[derive(Debug)]
pub struct Mem {
    db: Db,

    dispatcher_sender: UnboundedSender<MemToDispatcherCmd>,
    dispatcher_receiver: UnboundedReceiver<DispatcherToMemCmd>,
}

impl Mem {
    #[must_use]
    #[inline]
    pub fn new(
        dispatcher_sender: UnboundedSender<MemToDispatcherCmd>,
        dispatcher_receiver: UnboundedReceiver<DispatcherToMemCmd>,
    ) -> Self {
        Self {
            db: HashMap::new(),

            dispatcher_sender,
            dispatcher_receiver,
        }
    }
}
