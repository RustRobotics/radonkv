// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

#![allow(clippy::cast_possible_truncation)]

use std::ffi::c_void;
use std::mem::size_of_val;
use std::os::unix::io::{AsRawFd, RawFd};
use std::ptr;

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

    if !device.is_empty() {
        bind_device(socket_fd, device)?;
    }

    set_tcp_fastopen(socket_fd, 32)?;
    set_tcp_keepalive(socket_fd, 1)?;
    set_tcp_no_delay(socket_fd, 1)?;
    set_reuse_addr(socket_fd, 1)?;

    Ok(listener)
}

fn bind_device(socket_fd: RawFd, device: &str) -> Result<(), Error> {
    debug_assert!(!device.is_empty());
    let ret = unsafe {
        let socket_len = device.len() as nc::socklen_t;
        nc::setsockopt(
            socket_fd,
            nc::SOL_SOCKET,
            nc::SO_BINDTODEVICE,
            device.as_ptr().cast::<c_void>(),
            socket_len,
        )
    };
    if let Err(errno) = ret {
        Err(Error::from_string(
            ErrorKind::SocketError,
            format!(
                "Failed to bind device: {}, err: {}",
                device,
                nc::strerror(errno)
            ),
        ))
    } else {
        Ok(())
    }
}

fn set_tcp_fastopen(socket_fd: RawFd, queue_len: i32) -> Result<(), Error> {
    // For Linux, value is the queue length of pending packets.
    // For the others, just a boolean value for enable and disable.
    let ptr = ptr::addr_of!(queue_len) as *const c_void;
    let len = size_of_val(&queue_len) as u32;
    let ret = unsafe { nc::setsockopt(socket_fd, nc::IPPROTO_TCP, nc::TCP_FASTOPEN, ptr, len) };

    if let Err(errno) = ret {
        Err(Error::from_string(
            ErrorKind::SocketError,
            format!(
                "Failed to set socket fast-open falg, got err: {}",
                nc::strerror(errno)
            ),
        ))
    } else {
        Ok(())
    }
}

fn set_tcp_keepalive(socket_fd: RawFd, keepalive: i32) -> Result<(), Error> {
    let ptr = ptr::addr_of!(keepalive) as *const c_void;
    let len = size_of_val(&keepalive) as u32;
    let ret = unsafe { nc::setsockopt(socket_fd, nc::IPPROTO_TCP, nc::SO_KEEPALIVE, ptr, len) };

    if let Err(errno) = ret {
        Err(Error::from_string(
            ErrorKind::SocketError,
            format!(
                "Failed to set tcp keepalive flag, got err: {}",
                nc::strerror(errno)
            ),
        ))
    } else {
        Ok(())
    }
}

fn set_tcp_no_delay(socket_fd: RawFd, no_delay: i32) -> Result<(), Error> {
    let ptr = ptr::addr_of!(no_delay) as *const c_void;
    let len = size_of_val(&no_delay) as u32;
    let ret = unsafe { nc::setsockopt(socket_fd, nc::IPPROTO_TCP, nc::SO_KEEPALIVE, ptr, len) };

    if let Err(errno) = ret {
        Err(Error::from_string(
            ErrorKind::SocketError,
            format!(
                "Failed to set tcp no-delay flag, got err: {}",
                nc::strerror(errno)
            ),
        ))
    } else {
        Ok(())
    }
}

fn set_reuse_addr(socket_fd: RawFd, reuse: i32) -> Result<(), Error> {
    let ptr = ptr::addr_of!(reuse) as *const c_void;
    let len = size_of_val(&reuse) as u32;
    let ret = unsafe { nc::setsockopt(socket_fd, nc::IPPROTO_TCP, nc::SO_REUSEADDR, ptr, len) };

    if let Err(errno) = ret {
        Err(Error::from_string(
            ErrorKind::SocketError,
            format!(
                "Failed to set socket reuse-address flag, got err: {}",
                nc::strerror(errno)
            ),
        ))
    } else {
        Ok(())
    }
}
