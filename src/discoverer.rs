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
use crate::dns::message::Message;
use crate::message::QueryMessage;
use crate::query::Query;
use crate::service::Service;

/// Discoverer represents a discoverer.
pub struct Discoverer {
    services: Vec<Service>,
    transport_mgr: MulticastManager,
}

impl Discoverer {
    /// new creates a new discoverer.
    pub fn new() -> Arc<Mutex<Discoverer>> {
        let discoverer = Arc::new(Mutex::new(Discoverer {
            transport_mgr: MulticastManager::new(),
            services: Vec::new(),
        }));
        {
            let mut discoverer_lock = discoverer.lock().unwrap();
            discoverer_lock
                .transport_mgr
                .add_observer(discoverer.clone());
        } // discoverer_lock is dropped here
        discoverer
    }

    ///search queries the discoverer.
    pub fn search(&mut self, query: &Query) -> Result<(), std::io::Error> {
        let q = QueryMessage::new(query);
        match q.to_bytes() {
            Ok(bytes) => {
                let pkt = Packet::from_bytes(&bytes);
                return self.transport_mgr.notify(&pkt);
            }
            Err(e) => {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, e.message()));
            }
        }
    }

    /// services returns the services of the discoverer.
    pub fn services(&self) -> &Vec<Service> {
        &self.services
    }

    /// start starts the discoverer.
    pub fn start(&mut self) -> Result<(), std::io::Error> {
        if self.transport_mgr.is_running() {
            return Ok(());
        }
        let addrs = vec![MULTICAST_V6_ADDR, MULTICAST_V4_ADDR];
        let ret = self.transport_mgr.start(&addrs, PORT);
        if ret.is_err() {
            return ret;
        }
        Ok(())
    }

    /// stop stops the discoverer.
    pub fn stop(&mut self) -> Result<(), std::io::Error> {
        self.transport_mgr.stop()
    }
}

impl Observer for Discoverer {
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

impl Drop for Discoverer {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}
