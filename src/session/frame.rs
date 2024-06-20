// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use std::io::Cursor;

use bytes::{Buf, BytesMut};
use stdext::function_name;

use crate::cmd::frame::{Frame, ParseFrameError};
use crate::cmd::reply_frame::ReplyFrame;
use crate::cmd::Command;
use crate::commands::SessionToListenerCmd;
use crate::error::Error;
use crate::session::status::Status;
use crate::session::Session;

impl Session {
    pub(super) async fn read_frames(&mut self) -> Option<Vec<Frame>> {
        loop {
            // Try parsing frame from buffer.
            let mut frames = Vec::new();
            while !self.buffer.is_empty() {
                log::debug!("Will call parse_frame()");
                match self.try_parse_frame() {
                    Ok(Some(frame)) => {
                        log::debug!("Got new frame: {frame:?}");
                        frames.push(frame);
                    }
                    Ok(None) => break,
                    Err(err) => {
                        log::warn!("Invalid frame, err: {err:?}");
                        let reply_frame = ReplyFrame::ConstError("Invalid frame");
                        if let Err(err) = self.send_frames_to_client(vec![reply_frame]).await {
                            log::warn!("Failed to send error frame to client, err: {err:?}");
                        }
                        // TODO(Shaohua): Close socket.
                        return None;
                    }
                }
            }

            if !frames.is_empty() {
                log::debug!("Got frames: {frames:?}");
                return Some(frames);
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
                Ok(_n) => (),
                Err(err) => {
                    log::warn!(
                        "{} Failed to read from socket with id: {}, err: {err:?}",
                        function_name!(),
                        self.id
                    );
                    self.status = Status::Disconnected;
                    return None;
                }
            }
        }
    }

    pub(super) async fn handle_client_frames(&mut self, frames: Vec<Frame>) -> Result<(), Error> {
        let mut commands = Vec::new();
        for frame in frames {
            let command = match Command::try_from(frame) {
                Ok(command) => command,
                Err(err) => {
                    log::warn!(
                        "{}, Failed to parse command from frame, err: {err:?}",
                        function_name!()
                    );
                    return self
                        .send_frame_to_client(ReplyFrame::invalid_command())
                        .await;
                }
            };
            commands.push(command);
        }
        let cmd = SessionToListenerCmd::Request {
            session_id: self.id,
            commands,
        };
        log::debug!("{} send cmd to listener, cmd: {cmd:?}", function_name!());
        Ok(self.listener_sender.send(cmd).await?)
    }

    pub(super) async fn send_frames_to_client(
        &mut self,
        mut reply_frames: Vec<ReplyFrame>,
    ) -> Result<(), Error> {
        log::debug!(
            "{} length of reply_frames: {}",
            function_name!(),
            reply_frames.len()
        );
        self.pending_frames.append(&mut reply_frames);
        if Some(self.pending_frames.len()) == self.frames_read.front().copied() {
            // TODO(Shaohua): Call io::Write trait, do not convert to Bytes object.
            let mut bytes = BytesMut::new();
            for frame in &self.pending_frames {
                frame.to_bytes(&mut bytes);
            }
            self.stream.write(&bytes.freeze()).await?;
            self.pending_frames.clear();
            self.frames_read.pop_front();
            self.stream.flush().await?;
        }

        Ok(())
    }

    pub(super) async fn send_frame_to_client(
        &mut self,
        reply_frame: ReplyFrame,
    ) -> Result<(), Error> {
        log::debug!("{} reply_frame: {reply_frame:?}", function_name!());
        // TODO(Shaohua): Call io::Write trait, do not convert to Bytes object.
        let mut bytes = BytesMut::new();
        self.pending_frames.push(reply_frame);
        for frame in &self.pending_frames {
            frame.to_bytes(&mut bytes);
        }
        self.stream.write(&bytes.freeze()).await?;
        self.pending_frames.clear();
        self.frames_read.pop_front();
        self.stream.flush().await?;

        Ok(())
    }

    fn try_parse_frame(&mut self) -> Result<Option<Frame>, Error> {
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
