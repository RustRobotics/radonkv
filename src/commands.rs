// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::cluster_mgmt::ClusterManagementCommand;
use crate::cmd::Command;
use crate::cmd::reply_frame::ReplyFrame;
use crate::cmd::server_mgmt::ServerManagementCommand;
use crate::cmd::storage_mgmt::StorageManagementCommand;
use crate::listener::types::{SessionGroup, SessionId};

#[derive(Debug, Clone)]
pub struct ListenerToSessionCmd {
    pub session_id: SessionId,
    pub reply_frames: Vec<ReplyFrame>,
}

#[derive(Debug, Clone)]
pub enum SessionToListenerCmd {
    Request {
        session_id: SessionId,
        commands: Vec<Command>,
    },
    Disconnect(SessionId),
}

#[derive(Debug, Clone)]
pub struct ListenerToDispatcherCmd {
    pub session_group: SessionGroup,
    pub commands: Vec<Command>,
}

#[derive(Debug, Clone)]
pub struct DispatcherToListenerCmd {
    pub session_group: SessionGroup,
    pub reply_frames: Vec<ReplyFrame>,
}

#[derive(Debug, Clone)]
pub struct DispatcherToClusterCmd {
    pub session_group: SessionGroup,
    pub command: ClusterManagementCommand,
}

#[derive(Debug, Clone)]
pub struct ClusterToDispatcherCmd {
    pub session_group: SessionGroup,
    pub reply_frame: ReplyFrame,
}

#[derive(Debug, Clone)]
pub struct DispatcherToStorageCmd {
    pub session_group: SessionGroup,
    pub command: StorageManagementCommand,
}

#[derive(Debug, Clone)]
pub struct StorageToDispatcherCmd {
    pub session_group: SessionGroup,
    pub reply_frame: ReplyFrame,
}

#[derive(Debug, Clone)]
pub struct DispatcherToMemCmd {
    pub session_group: SessionGroup,
    pub commands: Vec<Command>,
}

#[derive(Debug, Clone)]
pub struct MemToDispatcherCmd {
    pub session_group: SessionGroup,
    pub reply_frames: Vec<ReplyFrame>,
}

#[derive(Debug, Clone)]
pub struct DispatcherToServerCmd {
    pub session_group: SessionGroup,
    pub command: ServerManagementCommand,
}

#[derive(Debug, Clone)]
pub struct ServerToDispatcherCmd {
    pub session_group: SessionGroup,
    pub reply_frame: ReplyFrame,
}
