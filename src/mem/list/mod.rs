// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use std::collections::LinkedList;

use crate::cmd::list::ListCommand;
use crate::cmd::reply_frame::ReplyFrame;
use crate::mem::Mem;
use crate::mem::util::prune_range;

mod index;
mod insert;
mod len;
mod pop_back;
mod pop_front;
mod push_back;
mod push_back_exist;
mod push_front;
mod push_front_exist;
mod range;
mod remove;
mod set;

pub type ListObject = LinkedList<Vec<u8>>;

impl Mem {
    #[allow(clippy::needless_pass_by_value)]
    pub fn handle_list_command(&mut self, command: ListCommand) -> ReplyFrame {
        match command {
            ListCommand::Index(key, index) => index::index(&self.db, &key, index),
            ListCommand::Insert(key, position, pivot, element) => {
                insert::insert(&mut self.db, key, position, pivot, element)
            }
            ListCommand::Len(key) => len::len(&self.db, &key),
            ListCommand::PushBack(key, values) => push_back::push_back(&mut self.db, key, values),
            ListCommand::PushBackExist(key, values) => {
                push_back_exist::push_back_exist(&mut self.db, &key, values)
            }
            ListCommand::PushFront(key, values) => {
                push_front::push_front(&mut self.db, key, values)
            }
            ListCommand::PushFrontExist(key, values) => {
                push_front_exist::push_front_exist(&mut self.db, &key, values)
            }
            ListCommand::PopBack(key, count) => pop_back::pop_back(&mut self.db, &key, count),
            ListCommand::PopFront(key, count) => pop_front::pop_front(&mut self.db, &key, count),
            ListCommand::Range(key, start, end) => range::range(&self.db, &key, start, end),
            ListCommand::Remove(key, count, element) => {
                remove::remove(&mut self.db, &key, count, element)
            }
            ListCommand::Set(key, index, value) => set::set(&mut self.db, &key, index, value),
        }
    }
}

pub fn to_reply_frame(list: &ListObject) -> ReplyFrame {
    let mut sub_list = Vec::new();
    for item in list.iter() {
        sub_list.push(ReplyFrame::Bulk(item.clone()));
    }
    ReplyFrame::Array(sub_list)
}

pub fn range_to_reply_frame(list: &ListObject, start: isize, end: isize) -> ReplyFrame {
    if let Some((start, end)) = prune_range(list.len(), start, end) {
        let mut sub_list = Vec::new();
        // FIXME(Shaohua): Check list range error.
        for item in list.iter().take(end + 1).skip(start) {
            sub_list.push(ReplyFrame::Bulk(item.clone()));
        }
        ReplyFrame::Array(sub_list)
    } else {
        ReplyFrame::EmptyArray
    }
}
