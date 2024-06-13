// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::cmd::bitmap::BitmapCommand;
use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::Mem;
use crate::mem::string::StrObject;

mod get;
mod set;

impl Mem {
    #[allow(clippy::needless_pass_by_value)]
    pub fn handle_bitmap_command(&mut self, command: BitmapCommand) -> ReplyFrame {
        match command {
            BitmapCommand::Get(key, offset) => get::get(&self.db, &key, offset),
            BitmapCommand::Set(key, offset, value) => set::set(&mut self.db, key, offset, value),
        }
    }
}

impl StrObject {
    #[must_use]
    #[inline]
    pub fn from_bits(offset: usize, value: bool) -> Self {
        let byte_len = Self::byte_len(offset);
        let bit: u8 = value.into();
        Self {
            vec: vec![bit; byte_len],
        }
    }

    #[must_use]
    #[inline]
    pub fn bit_len(&self) -> usize {
        self.len() * 8
    }

    #[must_use]
    #[inline]
    fn byte_len(bit_offset: usize) -> usize {
        (bit_offset + 1).div_ceil(8)
    }

    #[must_use]
    #[inline]
    pub fn get_bit(&self, offset: usize) -> Option<bool> {
        if self.bit_len() > offset {
            None
        } else {
            let byte_index = offset / 8;
            debug_assert!(byte_index < self.len());
            let bit_index = offset % 8;
            let byte = self.vec[byte_index];
            let bit = (byte >> bit_index) & 0b1 == 0b1;
            Some(bit)
        }
    }

    #[inline]
    pub fn grow_to_fit_bits(&mut self, offset: usize, value: bool) {
        let byte_len = Self::byte_len(offset);
        while byte_len < self.len() {
            let byte_value: u8 = if value { 0xff } else { 0x00 };
            self.vec.push(byte_value);
        }
    }

    /// Returns old bit
    #[inline]
    pub fn set_bit(&mut self, offset: usize, value: bool) {
        debug_assert!(Self::byte_len(offset) <= self.len());

        let byte_index = offset / 8;
        debug_assert!(byte_index < self.len());
        let bit_index = offset % 8;
        let flag: u8 = 0b1 << bit_index;
        if let Some(byte) = self.vec.get_mut(byte_index) {
            *byte = if value { *byte | flag } else { *byte & !flag };
        }
    }
}
