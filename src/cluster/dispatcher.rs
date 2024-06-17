// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use stdext::function_name;

use crate::cluster::Cluster;
use crate::commands::DispatcherToClusterCmd;
use crate::error::Error;

impl Cluster {
    #[allow(clippy::unused_async)]
    pub(super) async fn handle_dispatcher_cmd(
        &mut self,
        cmd: DispatcherToClusterCmd,
    ) -> Result<(), Error> {
        log::debug!("{} cmd: {cmd:?}", function_name!());
        Ok(())
    }
}
