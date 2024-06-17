// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::reply_frame::ReplyFrame;

/// Save the DB in background.
///
/// Normally the OK code is immediately returned. Redis forks, the parent continues
/// to serve the clients, the child saves the DB on disk then exits.
///
/// An error is returned if there is already a background save running or if there is
/// another non-background-save process running, specifically an in-progress AOF rewrite.
///
/// One of the following reply:
/// - Simple string reply: Background saving started.
/// - Simple string reply: Background saving scheduled.
pub fn background_save() -> ReplyFrame {
    log::info!("TODO(Shaohua): storage, impl background_save()");
    ReplyFrame::ConstSimple("Background saving started")
}
