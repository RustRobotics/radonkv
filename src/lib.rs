// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

#![deny(
    warnings,
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic
)]
#![allow(
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::multiple_crate_versions
)]
// TODO(Shaohua): Remove
#![allow(dead_code)]

pub mod cluster;
pub mod cmd;
mod commands;
pub mod config;
pub mod dispatcher;
pub mod error;
pub mod listener;
mod log;
pub mod mem;
pub mod server;
mod session;
pub mod storage;
