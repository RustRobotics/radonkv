// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use bloomfilter::Bloom;

use crate::cmd::bloom_filter::BloomFilterCommand;
use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::Mem;

mod add;

pub type BloomFilterObject = Bloom<String>;

impl Mem {
    pub fn handle_bloom_filter_command(&mut self, command: BloomFilterCommand) -> ReplyFrame {
        match command {
            BloomFilterCommand::Add(key, items) => add::add(&mut self.db, key, &items),
        }
    }
}

fn new_bloom_filter() -> BloomFilterObject {
    // TODO(Shaohua): Check size of bloom filter.
    Bloom::new(u32::MAX as usize, u32::MAX as usize / 2)
}
