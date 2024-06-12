// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use std::io::Cursor;

use bytes::{Buf, Bytes};
use stdext::function_name;

use crate::cmd::{Command, CommandCategory};
use crate::cmd::frame::{Frame, ParseFrameError};
use crate::cmd::reply_frame::ReplyFrame;
use crate::commands::SessionToListenerCmd;
use crate::error::Error;
use crate::session::Session;
use crate::session::status::Status;

impl Session {
    pub(super) async fn read_frame(&mut self) -> Option<Frame> {
        loop {
            // Try parsing frame from buffer.
            match self.parse_frame() {
                Ok(Some(frame)) => {
                    return Some(frame);
                }
                Ok(None) => (),
                Err(err) => {
                    log::warn!("Invalid frame, err: {err:?}");
                    let reply_frame = ReplyFrame::ConstError("Invalid frame");
                    if let Err(err) = self.send_frame_to_client(reply_frame).await {
                        log::warn!("Failed to send error frame to client, err: {err:?}");
                    }
                    // TODO(Shaohua): Close socket.
                    return None;
                }
            }

            match self.stream.read_buf(&mut self.buffer).await {
                Ok(0) => {
                    log::info!(
                        "{} Empty packet received, disconnect client, {}",
                        function_name!(),
                        self.id
                    );
                    self.status = Status::Disconnected;
                    return None;
                }
                Err(err) => {
                    log::warn!(
                        "{} Failed to read from socket with id: {}, err: {err:?}",
                        function_name!(),
                        self.id
                    );
                    self.status = Status::Disconnected;
                    return None;
                }
                _ => (),
            }
        }
    }

    pub(super) async fn handle_client_frame(&mut self, frame: Frame) -> Result<(), Error> {
        log::debug!("{}", function_name!());
        // 1. parse frame
        // 2.1. if frame is None, waiting for more data
        // 2.2. if frame is ok, parse command
        // 2.3. if frame is invalid, send failed to client
        // 3.1. if command is parsed ok, send that new cmd to listener
        // 3.2. else send error to client.
        match Command::try_from(frame) {
            Ok(command) => {
                if command.category() == CommandCategory::Session {
                    self.handle_client_command(command).await
                } else {
                    let cmd = SessionToListenerCmd::Cmd(self.id, command);
                    log::debug!("{} send cmd to listener, cmd: {cmd:?}", function_name!());
                    Ok(self.listener_sender.send(cmd).await?)
                }
            }
            Err(err) => {
                log::warn!("Invalid command, err: {err:?}");
                self.send_frame_to_client(ReplyFrame::invalid_command())
                    .await
            }
        }
    }

    pub(super) async fn send_frame_to_client(
        &mut self,
        reply_frame: ReplyFrame,
    ) -> Result<(), Error> {
        log::debug!("{} reply_frame: {reply_frame:?}", function_name!());
        // TODO(Shaohua): Call io::Write trait, do not convert to Bytes object.
        let bytes: Bytes = reply_frame.into_bytes();
        self.stream.write(&bytes).await?;
        // TODO(Shaohua): flush stream with pipeline
        //self.stream.flush().await
        Ok(())
    }

    fn parse_frame(&mut self) -> Result<Option<Frame>, Error> {
        let mut cursor = Cursor::new(&self.buffer[..]);
        match Frame::check_msg(&mut cursor) {
            Ok(()) => {
                let len =
                    usize::try_from(cursor.position()).map_err(Into::<ParseFrameError>::into)?;
                // Rewind to start.
                cursor.set_position(0);
                let frame = Frame::parse(&mut cursor)?;
                self.buffer.advance(len);
                Ok(Some(frame))
            }
            Err(ParseFrameError::Incomplete) => Ok(None),
            Err(err) => Err(err.into()),
        }
    }
}
