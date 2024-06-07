// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use stdext::function_name;

use crate::commands::DispatcherToStorageCmd;
use crate::error::Error;
use crate::storage::Storage;

impl Storage {
    #[allow(clippy::unused_async)]
    pub(super) async fn handle_dispatcher_cmd(
        &mut self,
        cmd: DispatcherToStorageCmd,
    ) -> Result<(), Error> {
        println!("{} cmd: {cmd:?}", function_name!());
        Ok(())
    }
}
