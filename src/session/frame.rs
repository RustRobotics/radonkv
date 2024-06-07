// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use std::io::Cursor;

use bytes::Buf;

use crate::cmd::frame::{Frame, ParsingFrameError};
use crate::commands::SessionToListenerCmd;
use crate::error::Error;
use crate::session::Session;

impl Session {
    pub(crate) async fn handle_client_frame(&mut self) -> Result<(), Error> {
        // 1. parse frame
        // 2.1. if frame is None, waiting for more data
        // 2.2. if frame is ok, send that new frame to listener
        // 2.3. if frame is invalid, send failed to client
        match self.parse_frame() {
            Ok(None) => {
                Ok(())
            }
            Ok(Some(frame)) => {
                self.listener_sender.send(SessionToListenerCmd::Frame(self.id, frame)).await?;
                Ok(())
            }
            Err(err) => {
                log::warn!("Invalid frame, err: {err:?}");
                // TODO(Shaohua):
                Ok(())
            }
        }
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