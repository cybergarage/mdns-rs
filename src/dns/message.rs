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

use std::fmt;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

pub struct Message {}

impl Message {
    pub fn new() -> Message {
        Message {}
    }

    pub fn from_bytes(msg_bytes: &[u8]) -> Message {
        let mut msg = Message::new();
        msg.parse(msg_bytes);
        msg
    }

    pub fn parse(&mut self, msg: &[u8]) -> bool {
        if !self.parse_header(msg) {
            return false;
        }
        true
    }

    fn parse_header(&mut self, header: &[u8]) -> bool {
        true
    }

    pub fn equals(&self, msg: &Message) -> bool {
        true
    }

    pub fn bytes(&self) -> Vec<u8> {
        let mut msg_bytes: Vec<u8> = Vec::new();
        msg_bytes
    }
}

impl Clone for Message {
    fn clone(&self) -> Message {
        let mut msg = Message::new();
        msg.parse(&self.bytes());
        msg
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}
