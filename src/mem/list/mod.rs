// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use std::collections::LinkedList;

use crate::cmd::list::ListCommand;
use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::Mem;

mod len;
mod push_front;
mod push_back;
mod pop_front;
mod pop_back;
mod push_back_exist;
mod push_front_exist;
mod range;

pub type ListObject = LinkedList<Vec<u8>>;

impl Mem {
    #[allow(clippy::needless_pass_by_value)]
    pub fn handle_list_command(&mut self, command: ListCommand) -> ReplyFrame {
        match command {
            ListCommand::Len(key) => len::len(&self.db, &key),
            ListCommand::PushBack(key, values) => push_back::push_back(&mut self.db, key, values),
            ListCommand::PushBackExist(key, values) => push_back_exist::push_back_exist(&mut self.db, key, values),
            ListCommand::PushFront(key, values) => push_front::push_front(&mut self.db, key, values),
            ListCommand::PushFrontExist(key, values) => push_front_exist::push_front_exist(&mut self.db, key, values),
            ListCommand::PopBack(key, count) => pop_back::pop_back(&mut self.db, key, count),
            ListCommand::PopFront(key, count) => pop_front::pop_front(&mut self.db, key, count),
            ListCommand::Range(key, start, end) => range::range(&self.db, &key, start, end),
        }
    }
}
