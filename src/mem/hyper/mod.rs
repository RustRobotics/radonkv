// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use std::hash::RandomState;

use hyperloglogplus::{HyperLogLogError, HyperLogLogPlus};

use crate::cmd::hyper::HyperLogLogCommand;
use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::Mem;

mod add;
mod count;
mod merge;

pub type HyperObject = HyperLogLogPlus<String, RandomState>;

#[inline]
fn new_hyper_object() -> Result<HyperObject, HyperLogLogError> {
    HyperLogLogPlus::new(18, RandomState::new())
}

impl Mem {
    #[allow(clippy::needless_pass_by_value)]
    pub fn handle_hyper_command(&mut self, command: HyperLogLogCommand) -> ReplyFrame {
        match command {
            HyperLogLogCommand::Add(key, elements) => add::add(&mut self.db, key, &elements),
            HyperLogLogCommand::Count(keys) => count::count(&mut self.db, &keys[0], &keys[1..]),
            HyperLogLogCommand::Merge(dest_key, source_keys) => {
                merge::merge(&mut self.db, dest_key, &source_keys)
            }
        }
    }
}

pub fn to_reply_frame(_hyper_object: &HyperObject) -> ReplyFrame {
    todo!()
}

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use crate::mem::hyper::HyperObject;

    #[test]
    fn test_hyper_log_log_object() {
        assert_eq!(size_of::<HyperObject>(), 32);
    }
}
