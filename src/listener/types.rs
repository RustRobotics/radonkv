// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

pub type SessionId = u64;
pub type ListenerId = u32;

/// Global session id.
///
/// It is a tuple of `(listener_id, session_id)`.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SessionGid {
    listener_id: ListenerId,
    session_id: SessionId,
}

impl SessionGid {
    #[must_use]
    pub const fn new(listener_id: ListenerId, session_id: SessionId) -> Self {
        Self {
            listener_id,
            session_id,
        }
    }

    /// Get listener id.
    #[must_use]
    #[inline]
    pub const fn listener_id(&self) -> ListenerId {
        self.listener_id
    }

    /// Get session id.
    #[must_use]
    #[inline]
    pub const fn session_id(&self) -> SessionId {
        self.session_id
    }
}
