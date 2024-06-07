// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use stdext::function_name;

use crate::commands::StorageToDispatcherCmd;
use crate::dispatcher::Dispatcher;

impl Dispatcher {
    #[allow(clippy::unused_async)]
    pub(super) async fn handle_storage_cmd(&mut self, cmd: StorageToDispatcherCmd) {
        println!("{} cmd: {cmd:?}", function_name!());
    }
}
