// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use std::os::unix::io::{AsRawFd, RawFd};

use tokio::net::TcpListener;

use crate::error::{Error, ErrorKind};

/// Create a new tcp server socket at `address` and binds to `device`.
///
/// # Errors
///
/// Returns error if socket `address` is invalid or failed to bind to specific `device`.
pub async fn new_tcp_listener(address: &str, device: &str) -> Result<TcpListener, Error> {
    let listener = TcpListener::bind(address).await?;
    let socket_fd: RawFd = listener.as_raw_fd();

    bind_device(socket_fd, device)?;
    enable_fast_open(socket_fd)?;

    // TODO(Shaohua): Tuning tcp keep alive flag.
    // TODO(Shaohua): Tuning cpu affinity flag.

    Ok(listener)
}

fn bind_device(socket_fd: RawFd, device: &str) -> Result<(), Error> {
    if !device.is_empty() {
        unsafe {
            #[allow(clippy::cast_possible_truncation)]
            let socket_len = device.len() as nc::socklen_t;
            nc::setsockopt(
                socket_fd,
                nc::SOL_SOCKET,
                nc::SO_BINDTODEVICE,
                device.as_ptr() as usize,
                socket_len,
            )
            .map_err(|errno| {
                Error::from_string(
                    ErrorKind::KernelError,
                    format!(
                        "Failed to bind device: {}, err: {}",
                        device,
                        nc::strerror(errno)
                    ),
                )
            })?;
        }
    }
    Ok(())
}

fn enable_fast_open(socket_fd: RawFd) -> Result<(), Error> {
    // For Linux, value is the queue length of pending packets.
    //
    // TODO(Shaohua): Add a config option
    let queue_len: i32 = 5;
    // For the others, just a boolean value for enable and disable.
    let queue_len_ptr = std::ptr::addr_of!(queue_len) as usize;

    unsafe {
        #[allow(clippy::cast_possible_truncation)]
        let len = std::mem::size_of_val(&queue_len) as u32;
        nc::setsockopt(
            socket_fd,
            nc::IPPROTO_TCP,
            nc::TCP_FASTOPEN,
            queue_len_ptr,
            len,
        )
        .map_err(|errno| {
            Error::from_string(
                ErrorKind::KernelError,
                format!(
                    "Failed to enable socket fast open, got err: {}",
                    nc::strerror(errno)
                ),
            )
        })
    }
}
