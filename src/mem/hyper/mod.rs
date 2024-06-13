// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use std::hash::RandomState;

use hyperloglogplus::HyperLogLogPlus;

use crate::cmd::hyper::HyperLogLogCommand;
use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::Mem;

mod count;

pub type HyperObject = HyperLogLogPlus<u64, RandomState>;

impl Mem {
    #[allow(clippy::needless_pass_by_value)]
    pub fn handle_hyper_command(&mut self, command: HyperLogLogCommand) -> ReplyFrame {
        match command {
            HyperLogLogCommand::Count(primary_key, extra_keys) => {
                count::count(&mut self.db, &primary_key, &extra_keys)
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
