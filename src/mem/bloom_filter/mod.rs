// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use bloomfilter::Bloom;

use crate::cmd::bloom_filter::BloomFilterCommand;
use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::Mem;

mod add;
mod exists;
mod len;
mod multi_add;

#[derive(Debug, Clone)]
pub struct BloomFilterObject {
    len: usize,
    bloom: Bloom<String>,
}

impl Mem {
    pub fn handle_bloom_filter_command(&mut self, command: BloomFilterCommand) -> ReplyFrame {
        match command {
            BloomFilterCommand::Add(key, item) => add::add(&mut self.db, key, &item),
            BloomFilterCommand::MultiAdd(key, items) => {
                multi_add::multi_add(&mut self.db, key, &items)
            }
            BloomFilterCommand::Len(key) => len::len(&self.db, &key),
            BloomFilterCommand::Exists(key, item) => exists::exists(&self.db, &key, &item),
        }
    }
}

impl BloomFilterObject {
    #[must_use]
    #[inline]
    pub fn new() -> Self {
        // TODO(Shaohua): Check size of bloom filter.
        let bloom = Bloom::new(u32::MAX as usize, u32::MAX as usize / 2);
        Self { len: 0, bloom }
    }

    #[must_use]
    #[inline]
    pub fn check_and_set(&mut self, item: &String) -> bool {
        let is_set = self.bloom.check_and_set(item);
        self.len += 1;
        is_set
    }

    #[must_use]
    #[inline]
    pub fn check(&self, item: &String) -> bool {
        self.bloom.check(item)
    }

    #[must_use]
    #[inline]
    pub const fn len(&self) -> usize {
        self.len
    }
}
