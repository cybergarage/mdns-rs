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

use cybergarage::net::{MulticastManager, Observer, Packet};

use crate::default::{MULTICAST_V4_ADDR, MULTICAST_V6_ADDR, PORT};
use crate::discoverer::Discoverer;
use crate::dns::message::Message;
use crate::message::QueryMessage;
use crate::query::Query;
use crate::service::Service;

/// Client represents a client.
pub struct Client {
    discoverer: Arc<Mutex<Discoverer>>,
}

impl Client {
    /// new creates a new client.
    pub fn new() -> Client {
        Client {
            discoverer: Discoverer::new(),
        }
    }

    ///search queries the client.
    pub fn search(&mut self, query: &Query) -> bool {
        self.discoverer.lock().unwrap().search(query)
    }

    /// start starts the client.
    pub fn start(&mut self) -> bool {
        self.discoverer.lock().unwrap().start()
    }

    /// stop stops the client.
    pub fn stop(&mut self) -> bool {
        self.discoverer.lock().unwrap().stop()
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        self.stop();
    }
}
