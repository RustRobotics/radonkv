// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use stdext::function_name;

use crate::commands::ListenerToDispatcherCmd;
use crate::dispatcher::Dispatcher;

impl Dispatcher {
    pub(super) async fn handle_listener_cmd(&mut self, cmd: ListenerToDispatcherCmd) {
        println!("{}, cmd: {cmd:?}", function_name!());
    }
}