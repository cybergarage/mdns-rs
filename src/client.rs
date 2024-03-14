// Copyright (C) 2024 Satoshi Konno All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::sync::Arc;
use std::sync::Mutex;

use cybergarage::net::MulticastManager;

use crate::default::{MULTICAST_V4_ADDR, MULTICAST_V6_ADDR, PORT};

pub struct Client {
    transport_mgr: MulticastManager,
}

impl Client {
    pub fn new() -> Arc<Mutex<Client>> {
        // let (tx, _): (Sender<Packet>, Receiver<Packet>) = mpsc::channel();
        let client = Arc::new(Mutex::new(Client {
            transport_mgr: MulticastManager::new(),
        }));
        client
    }

    pub fn is_running(&self) -> bool {
        self.transport_mgr.is_running()
    }

    pub fn start(&mut self) -> bool {
        if self.transport_mgr.is_running() {
            return true;
        }
        let addrs = vec![MULTICAST_V6_ADDR, MULTICAST_V4_ADDR];
        if !self.transport_mgr.start(&addrs, PORT) {
            return false;
        }
        true
    }

    pub fn stop(&mut self) -> bool {
        if !self.transport_mgr.stop() {
            return false;
        }
        true
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        self.stop();
    }
}
