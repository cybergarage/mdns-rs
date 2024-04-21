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
use crate::dns::{Message, QuestionRecord, Record, ResourceRecords, Type};
use crate::query::Query;
use crate::service::Service;

/// Client represents a client.
pub struct Client {
    services: Vec<Service>,
    transport_mgr: MulticastManager,
}

impl Client {
    /// new creates a new client.
    pub fn new() -> Arc<Mutex<Client>> {
        let client = Arc::new(Mutex::new(Client {
            transport_mgr: MulticastManager::new(),
            services: Vec::new(),
        }));
        {
            let mut client_lock = client.lock().unwrap();
            client_lock.transport_mgr.add_observer(client.clone());
        } // client_lock is dropped here
        client
    }

    ///query queries the client.
    pub fn query(&mut self, query: &Query) {
        let mut q = QuestionRecord::new();
        q.set_name(&query.to_string());
    }

    /// is_running returns true if the client is running.
    pub fn is_running(&self) -> bool {
        self.transport_mgr.is_running()
    }

    /// start starts the client.
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

    /// stop stops the client.
    pub fn stop(&mut self) -> bool {
        if !self.transport_mgr.stop() {
            return false;
        }
        true
    }
}

impl Observer for Client {
    fn packet_received(&mut self, pkt: &Packet) {
        let msg = Message::from_bytes(pkt.bytes());
        match msg {
            Ok(msg) => {
                let service = Service::from_message(&msg);
                self.services.push(service);
            }
            Err(_) => {
                return;
            }
        }
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        self.stop();
    }
}
