// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use crate::listener::Listener;

impl Listener {
    pub async fn run_loop(&mut self) -> ! {
        let mut session_receiver = self.session_receiver.take().expect("Invalid session receiver");
        let mut dispatcher_receiver = self.dispatcher_receiver.take().expect("Invalid dispatcher receiver");

        loop {
            tokio::select! {
                Ok(stream) = self.accept() => {
                    self.new_connection(stream).await;
                }
                Some(_cmd) = session_receiver.recv() => {
                    todo!()
                }
                Some(_cmd) = dispatcher_receiver.recv() => {
                    todo!()
                }
            }
        }
    }
}