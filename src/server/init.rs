// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use stdext::function_name;

use crate::error::Error;
use crate::server::context::ServerContext;

impl ServerContext {
    pub(crate) async fn init_modules(&mut self) -> Result<(), Error> {
        log::info!("{}", function_name!());

        Ok(())
    }
}