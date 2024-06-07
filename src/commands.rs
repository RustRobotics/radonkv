// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::Command;
use crate::cmd::frame::Frame;
use crate::listener::types::{SessionGroup, SessionId};

#[derive(Debug, Clone)]
pub enum ListenerToSessionCmd {
    Reply(SessionId, Frame),
}

#[derive(Debug, Clone)]
pub enum SessionToListenerCmd {
    Cmd(SessionId, Command),
    Disconnect(SessionId),
}

#[derive(Debug, Clone)]
pub enum ListenerToDispatcherCmd {
    Cmd(SessionGroup, Command),
}

#[derive(Debug, Clone)]
pub enum DispatcherToListenerCmd {
    Reply(SessionGroup, Frame),
}

#[derive(Debug, Clone)]
pub enum DispatcherToStorageCmd {
    Flush,
}

#[derive(Debug, Clone)]
pub enum StorageToDispatcherCmd {}

#[derive(Debug, Clone)]
pub struct DispatcherToMemCmd {
    pub session_group: SessionGroup,
    pub command: Command,
}

#[derive(Debug, Clone)]
pub struct MemToDispatcherCmd {
    pub session_group: SessionGroup,
    pub frame: Frame,
}
