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

use crate::dns::{AAAARecord, ARecord, Message, Record, ResourceRecords, Type};
use std::collections::HashMap;
use std::net::IpAddr;

/// Service represents a DNS-SD service.
pub struct Service {
    msg: Message,
    name: String,
    domain: String,
    host: String,
    ipaddrs: Vec<IpAddr>,
    port: u16,
    attrs: HashMap<String, String>,
}

impl Service {
    /// from_message creates a new Service from the specified message.
    pub fn from_message(msg: &Message) -> Service {
        let mut srv = Service {
            msg: msg.clone(),
            name: String::new(),
            domain: String::new(),
            host: String::new(),
            port: 0,
            ipaddrs: Vec::new(),
            attrs: HashMap::new(),
        };
        srv.parse_message(msg);
        srv
    }

    /// message returns the message of the service.
    pub fn message(&self) -> &Message {
        &self.msg
    }

    /// resource_records returns the resource records of the service.
    pub fn resource_records(&self) -> ResourceRecords {
        self.msg.resource_records()
    }

    /// name returns the name of the service.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// domain returns the domain of the service.
    pub fn domain(&self) -> &str {
        &self.domain
    }

    /// host returns the host of the service.
    pub fn host(&self) -> &str {
        &self.host
    }

    /// ipaddrs returns the IP addresses of the service.
    pub fn ipaddrs(&self) -> &Vec<IpAddr> {
        &self.ipaddrs
    }

    /// port returns the port of the service.
    pub fn port(&self) -> u16 {
        self.port
    }

    /// attributes returns the attributes of the service.
    pub fn attributes(&self) -> &HashMap<String, String> {
        &self.attrs
    }

    /// attribute returns the attribute of the service.
    pub fn attribute(&self, key: &str) -> Option<&String> {
        self.attrs.get(key)
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
            Type::SRV => {
                let srv = crate::dns::SRVRecord::from_record(record).unwrap();
                self.name = srv.name().to_string();
                self.domain = srv.proto().to_string();
                self.host = srv.target().to_string();
                self.port = srv.port();
            }
            Type::TXT => {
                let txt = crate::dns::TXTRecord::from_record(record).unwrap();
                self.attrs = txt.attributes().clone();
            }
            _ => {}
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
        }
    }

    pub fn to_string(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!("name: {}\n", self.name));
        s.push_str(&format!("domain: {}\n", self.domain));
        s.push_str(&format!("host: {}\n", self.host));
        s.push_str(&format!("port: {}\n", self.port));
        for ipaddr in &self.ipaddrs {
            s.push_str(&format!("ipaddr: {}\n", ipaddr));
        }
        for (key, value) in &self.attrs {
            s.push_str(&format!("{}: {}\n", key, value));
        }
        s
    }
}
