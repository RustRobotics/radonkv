// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::Command;
use crate::cmd::conn::ConnectManagementCommand;
use crate::cmd::reply_frame::ReplyFrame;
use crate::error::Error;
use crate::listener::types::SessionId;
use crate::session::Session;

impl Session {
    pub(super) async fn handle_client_command(&mut self, command: Command) -> Result<(), Error> {
        if let Command::ConnManagement(cmd) = command {
            let reply_frame = match cmd {
                ConnectManagementCommand::GetId() => Self::get_id_command(self.id),
                ConnectManagementCommand::Echo(message) => Self::echo_command(message),
                ConnectManagementCommand::Ping(message) => Self::ping_command(message),
                ConnectManagementCommand::GetName() => Self::get_name_command(self.name.as_ref()),
                ConnectManagementCommand::SetName(new_name) => self.set_name_command(new_name),
            };
            self.send_frame_to_client(reply_frame).await
        } else {
            unreachable!()
        }
    }

    /// The `CLIENT GETNAME` returns the name of the current connection as
    /// set by `CLIENT SETNAME`.
    ///
    /// Since every new connection starts without an associated name,
    /// if no name was assigned a null bulk reply is returned.
    ///
    /// One of the following reply:
    /// - Bulk string reply: the connection name of the current connection.
    /// - Null reply: the connection name was not set.

    pub fn get_name_command(old_name: Option<&String>) -> ReplyFrame {
        old_name.map_or(ReplyFrame::Null, |name| {
            ReplyFrame::Bulk(name.as_bytes().to_vec())
        })
    }

    /// The `CLIENT SETNAME` command assigns a name to the current connection.
    ///
    /// Reply:
    /// - Simple string reply: OK if the connection name was successfully set.
    pub fn set_name_command(&mut self, new_name: String) -> ReplyFrame {
        self.name = Some(new_name);
        ReplyFrame::ok()
    }

    /// The command just returns the ID of the current connection.
    ///
    /// Reply:
    /// - Integer reply: the ID of the client.
    #[must_use]
    #[inline]
    pub const fn get_id_command(id: SessionId) -> ReplyFrame {
        ReplyFrame::I64(id)
    }

    /// Returns PONG if no argument is provided, otherwise return a copy of the argument as a bulk.
    ///
    /// This command is useful for:
    /// - Testing whether a connection is still alive.
    /// - Verifying the server's ability to serve data - an error is returned when this isn't the case (e.g., during load from persistence or accessing a stale replica).
    /// - Measuring latency.
    ///
    /// If the client is subscribed to a channel or a pattern, it will instead return a multi-bulk
    /// with a "pong" in the first position and an empty bulk in the second position,
    /// unless an argument is provided in which case it returns a copy of the argument.
    ///
    /// Any of the following reply:
    /// - Simple string reply: PONG when no argument is provided.
    /// - Bulk string reply: the provided argument.
    pub fn ping_command(message: Option<String>) -> ReplyFrame {
        message.map_or_else(ReplyFrame::pong, |message| {
            ReplyFrame::Bulk(message.into_bytes())
        })
    }

    /// Returns message.
    ///
    /// Reply:
    /// - Bulk string reply: the given string.
    pub fn echo_command(message: String) -> ReplyFrame {
        ReplyFrame::Bulk(message.into_bytes())
    }
}

#[cfg(test)]
mod tests {
    use crate::cmd::reply_frame::ReplyFrame;
    use crate::session::Session;

    #[test]
    fn test_ping_command() {
        let reply = Session::ping_command(None);
        assert_eq!(reply, ReplyFrame::pong());
        let reply = Session::ping_command(Some("hello world".to_owned()));
        assert_eq!(reply, ReplyFrame::bulk(b"hello world".to_vec()));
    }

    #[test]
    fn test_echo_command() {
        let reply = Session::echo_command("hello world".to_owned());
        assert_eq!(reply, ReplyFrame::bulk(b"hello world".to_vec()));
    }
}
