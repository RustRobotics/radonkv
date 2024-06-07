// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::Command;
use crate::cmd::frame::Frame;
use crate::listener::types::{SessionGid, SessionId};

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
    Cmd(SessionGid, Command),
}

#[derive(Debug, Clone)]
pub enum DispatcherToListenerCmd {
    Reply(SessionGid, Frame),
}

#[derive(Debug, Clone)]
pub enum DispatcherToStorageCmd {
    Flush,
}

#[derive(Debug, Clone)]
pub enum StorageToDispatcherCmd {}

#[derive(Debug, Clone)]
pub struct DispatcherToMemCmd {
    pub session_gid: SessionGid,
    pub command: Command,
}

#[derive(Debug, Clone)]
pub struct MemToDispatcherCmd {
    pub session_gid: SessionGid,
    pub frame: Frame,
}