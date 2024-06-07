// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use std::io::Cursor;

use bytes::{Buf, Bytes};

use crate::cmd::Command;
use crate::cmd::frame::{Frame, ParsingFrameError};
use crate::commands::SessionToListenerCmd;
use crate::error::Error;
use crate::session::Session;

impl Session {
    pub(crate) async fn handle_client_frame(&mut self) -> Result<(), Error> {
        // 1. parse frame
        // 2.1. if frame is None, waiting for more data
        // 2.2. if frame is ok, parse command
        // 2.3. if frame is invalid, send failed to client
        // 3.1. if command is parsed ok, send that new cmd to listener
        // 3.2. else send error to client.
        match self.parse_frame() {
            Ok(None) => {
                Ok(())
            }
            Ok(Some(frame)) =>
                match Command::try_from(frame) {
                    Ok(command) => {
                        let cmd = SessionToListenerCmd::Cmd(self.id, command);
                        self.listener_sender.send(cmd).await?;
                        Ok(())
                    }
                    Err(err) => {
                        log::warn!("Invalid command, err: {err:?}");
                        self.send_frame_to_client(Frame::Error("Invalid command".to_owned())).await
                    }
                }
            Err(err) => {
                log::warn!("Invalid frame, err: {err:?}");
                self.send_frame_to_client(Frame::Error("Invalid frame".to_owned())).await
            }
        }
    }

    pub async fn send_frame_to_client(&mut self, frame: Frame) -> Result<(), Error> {
        let bytes: Bytes = frame.into_bytes();
        self.stream.write(&bytes).await?;
        self.stream.flush().await
    }

    pub(crate) async fn send_disconnect(&mut self) -> Result<(), Error> {
        todo!()
    }

    fn parse_frame(&mut self) -> Result<Option<Frame>, Error> {
        let mut cursor = Cursor::new(&self.buffer[..]);
        match Frame::check_msg(&mut cursor) {
            Ok(()) => {
                let len = cursor.position() as usize;
                // Rewind to start.
                cursor.set_position(0);
                let frame = Frame::parse(&mut cursor)?;
                self.buffer.advance(len);
                Ok(Some(frame))
            }
            Err(ParsingFrameError::Incomplete) => Ok(None),
            Err(err) => Err(err.into()),
        }
    }
}