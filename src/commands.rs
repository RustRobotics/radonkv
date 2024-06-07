// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::frame::Frame;
use crate::listener::types::{ListenerId, SessionId};

#[derive(Debug, Clone)]
pub enum ListenerToSessionCmd {}

#[derive(Debug, Clone)]
pub enum SessionToListenerCmd {
    Disconnect(SessionId),
}

#[derive(Debug, Clone)]
pub enum ListenerToDispatcherCmd {
    Get(ListenerId, String),
}

#[derive(Debug, Clone)]
pub enum DispatcherToListenerCmd {
    Reply(ListenerId, Frame),
}

#[derive(Debug, Clone)]
pub enum DispatcherToStorageCmd {
    Flush,
}

#[derive(Debug, Clone)]
pub enum StorageToDispatcherCmd {}