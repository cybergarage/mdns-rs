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

use crate::dns::error::MessageError;
use std::fmt;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

const HEADER_SIZE: usize = 12;

pub enum QR {
    Query = 0,
    Response = 1,
}

pub enum Opcode {
    Query = 0,
    IQuery = 1,
    Status = 2,
}

pub enum ResponseCode {
    NoError = 0,
    FormatError = 1,
    ServerFailure = 2,
    NameError = 3,
    NotImplemented = 4,
    Refused = 5,
}

pub struct Message {
    header: [u8; HEADER_SIZE],
}

impl Message {
    pub fn new() -> Message {
        Message {
            header: [0; HEADER_SIZE],
        }
    }

    pub fn from_bytes(msg_bytes: &[u8]) -> Result<Message, MessageError> {
        let mut msg = Message::new();
        let ret = msg.parse(msg_bytes);
        if ret.is_err() {
            return Err(ret.err().unwrap());
        };
        Ok(msg)
    }

    /// ID returns the query identifier.
    /// RFC 6762: 18.1. ID (Query Identifier)
    /// In multicast query messages, the Query Identifier SHOULD be set to zero on transmission.
    /// In multicast responses, including unsolicited multicast responses, the Query Identifier MUST be set to zero on transmission, and MUST be ignored on reception.
    pub fn ID(&self) -> u16 {
        ((self.header[0] as u16) << 8) | (self.header[1] as u16)
    }

    /// QR returns the query type.
    /// RFC 6762: 18.2. QR (Query/Response) Bit
    /// In query messages the QR bit MUST be zero. In response messages the QR bit MUST be one.
    pub fn QR(&self) -> QR {
        if (self.header[2] & 0x80) == 0 {
            return QR::Query;
        }
        QR::Response
    }

    pub fn parse(&mut self, msg_bytes: &[u8]) -> Result<(), MessageError> {
        let ret = self.parse_header(msg_bytes);
        if ret.is_err() {
            return Err(ret.err().unwrap());
        };
        Ok(())
    }

    fn parse_header(&mut self, header_bytes: &[u8]) -> Result<(), MessageError> {
        if header_bytes.len() < HEADER_SIZE {
            return Err(MessageError::new(header_bytes, 0));
        }
        self.header.copy_from_slice(header_bytes);
        Ok(())
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
