// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Log {
    /// Print logs to console.
    ///
    /// Default is true.
    #[serde(default = "Log::default_console_log")]
    console_log: bool,

    /// Set minimum log level.
    ///
    /// Available values are:
    /// - off, to disable log,
    /// - trace
    /// - debug
    /// - info
    /// - warn
    /// - error
    ///
    /// Default is "info" in release build and "trace" in debug build.
    #[serde(default = "Log::default_log_level")]
    log_level: LogLevel,

    /// Path to log file.
    ///
    /// Default is None.
    #[serde(default = "Log::default_log_file")]
    log_file: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Copy)]
pub enum LogLevel {
    #[serde(alias = "off")]
    Off,

    #[serde(alias = "error")]
    Error,

    #[serde(alias = "warn")]
    Warn,

    #[serde(alias = "info")]
    Info,

    #[serde(alias = "debug")]
    Debug,

    #[serde(alias = "trace")]
    Trace,
}

impl Default for Log {
    fn default() -> Self {
        Self {
            console_log: Self::default_console_log(),
            log_level: Self::default_log_level(),
            log_file: Self::default_log_file(),
        }
    }
}

impl Log {
    #[must_use]
    #[inline]
    pub const fn console_log(&self) -> bool {
        self.console_log
    }

    #[must_use]
    #[inline]
    pub const fn log_level(&self) -> LogLevel {
        self.log_level
    }

    #[must_use]
    #[inline]
    pub fn log_file(&self) -> Option<&String> {
        self.log_file.as_ref()
    }

    #[must_use]
    #[inline]
    pub const fn default_console_log() -> bool {
        true
    }

    #[must_use]
    #[inline]
    pub const fn default_log_level() -> LogLevel {
        #[cfg(debug_assertions)]
        return LogLevel::Debug;

        #[cfg(not(debug_assertions))]
        return LogLevel::Info;
    }

    #[must_use]
    #[inline]
    pub const fn default_log_file() -> Option<String> {
        None
    }
}
