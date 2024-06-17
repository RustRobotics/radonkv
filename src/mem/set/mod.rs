// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use std::collections::BTreeSet;

use crate::cmd::reply_frame::ReplyFrame;
use crate::cmd::set::SetCommand;
use crate::mem::Mem;

pub mod add;
pub mod diff;
pub mod intersect;
pub mod is_member;
pub mod len;
pub mod members;
pub mod random_member;
pub mod remove;
pub mod union;

pub type SetObject = BTreeSet<Vec<u8>>;

impl Mem {
    pub fn handle_set_command(&mut self, command: SetCommand) -> ReplyFrame {
        match command {
            SetCommand::Add(key, members) => add::add(&mut self.db, key, members),
            SetCommand::Len(key) => len::len(&self.db, &key),
            SetCommand::Members(key) => members::members(&self.db, &key),
            SetCommand::IsMember(key, member) => is_member::is_member(&self.db, &key, &member),
            SetCommand::Remove(key, members) => remove::remove(&mut self.db, &key, &members),
            SetCommand::RandomMember(key, count) => {
                random_member::random_member(&self.db, &key, count)
            }
            SetCommand::Intersect(keys) => intersect::intersect(&self.db, &keys),
            SetCommand::Union(keys) => union::union(&self.db, &keys),
            SetCommand::Diff(keys) => diff::diff(&self.db, &keys),
        }
    }
}
