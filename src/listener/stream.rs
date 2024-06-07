// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use std::net::SocketAddr;

use bytes::{Bytes, BytesMut};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::error::Error;

#[derive(Debug)]
pub enum Stream {
    Tcp(TcpStream, SocketAddr),
}

impl Stream {
    /// Read from stream.
    ///
    /// # Errors
    ///
    /// Returns error if stream/socket gets error.
    pub async fn read_buf(&mut self, buf: &mut BytesMut) -> Result<usize, Error> {
        match self {
            Self::Tcp(ref mut tcp_stream, _address) => Ok(tcp_stream.read_buf(buf).await?),
        }
    }

    /// Write buffer to stream.
    ///
    /// # Errors
    ///
    /// Returns error if socket/stream gets error.
    pub async fn write(&mut self, buf: &Bytes) -> Result<usize, Error> {
        match self {
            Self::Tcp(tcp_stream, _address) => Ok(tcp_stream.write(buf).await?),
        }
    }

    pub async fn flush(&mut self) -> Result<(), Error> {
        match self {
            Self::Tcp(tcp_stream, _address) =>
                Ok(tcp_stream.flush().await?),
        }
    }
}
