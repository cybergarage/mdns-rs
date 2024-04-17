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

use crate::dns::{Message, Record};
use std::net::IpAddr;

pub struct Service<'a> {
    message: &'a Message,
    name: String,
    domain: String,
    host: String,
    address: Vec<IpAddr>,
    port: u16,
}

impl Service<'_> {
    pub fn from_message(msg: &Message) -> Service {
        let mut srv = Service {
            message: msg,
            name: String::new(),
            domain: String::new(),
            host: String::new(),
            port: 0,
            address: Vec::new(),
        };
        srv.parse_message(msg);
        srv
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

    fn parse_record(&mut self, record: &Record) {}
}
