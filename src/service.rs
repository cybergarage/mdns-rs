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

use crate::dns::{AAAARecord, ARecord, Message, Record, Type};
use std::net::IpAddr;

pub struct Service {
    name: String,
    domain: String,
    host: String,
    ipaddrs: Vec<IpAddr>,
    port: u16,
}

impl Service {
    pub fn from_message(msg: &Message) -> Service {
        let mut srv = Service {
            name: String::new(),
            domain: String::new(),
            host: String::new(),
            port: 0,
            ipaddrs: Vec::new(),
        };
        srv.parse_message(msg);
        srv
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn domain(&self) -> &str {
        &self.domain
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn ipaddrs(&self) -> &Vec<IpAddr> {
        &self.ipaddrs
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    fn parse_message(&mut self, msg: &Message) {
        for record in msg.questions() {
            self.parse_record(record);
        }
        for record in msg.answers() {
            self.parse_record(record);
        }
        for record in msg.authorities() {
            self.parse_record(record);
        }
        for record in msg.additionals() {
            self.parse_record(record);
        }
    }

    fn parse_record(&mut self, record: &Record) {
        let data = record.data();
        match record.typ() {
            Type::A => match ARecord::from_record(record) {
                Ok(a) => {
                    self.ipaddrs.push(a.ipaddr().clone());
                }
                _ => {}
            },
            Type::AAAA => match AAAARecord::from_record(record) {
                Ok(a) => {
                    self.ipaddrs.push(a.ipaddr().clone());
                }
                _ => {}
            },
            _ => {}
        }
    }
}
