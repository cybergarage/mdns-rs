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

use crate::dns::error::Error;
use crate::dns::reader::Reader;
use crate::dns::record::Record;

const HEADER_SIZE: usize = 12;

/// QR represents the query type.
#[derive(PartialEq)]
pub enum QR {
    Query = 0,
    Response = 1,
}

/// Opcode represents the kind of query.
pub enum Opcode {
    Query = 0,
    IQuery = 1,
    Status = 2,
}

/// ResponseCode represents the response code.
pub enum ResponseCode {
    NoError = 0,
    FormatError = 1,
    ServerFailure = 2,
    NameError = 3,
    NotImplemented = 4,
    Refused = 5,
}

/// Message represents a DNS message.
pub struct Message {
    header: [u8; HEADER_SIZE],
    questions: Vec<Record>,
    answers: Vec<Record>,
    authorities: Vec<Record>,
    additionals: Vec<Record>,
}

/// Message represents a DNS message.
impl Message {
    /// new creates a new message.
    pub fn new() -> Message {
        Message {
            header: [0; HEADER_SIZE],
            questions: Vec::new(),
            answers: Vec::new(),
            authorities: Vec::new(),
            additionals: Vec::new(),
        }
    }

    /// from_bytes creates a new message from the specified bytes.
    pub fn from_bytes(msg_bytes: &[u8]) -> Result<Message, Error> {
        let mut msg = Message::new();
        let ret = msg.parse_bytes(msg_bytes);
        if ret.is_err() {
            return Err(ret.err().unwrap());
        };
        Ok(msg)
    }

    /// id returns the query identifier.
    /// RFC 6762: 18.1. ID (Query Identifier)
    /// In multicast query messages, the Query Identifier SHOULD be set to zero on transmission.
    /// In multicast responses, including unsolicited multicast responses, the Query Identifier MUST be set to zero on transmission, and MUST be ignored on reception.
    pub fn id(&self) -> u16 {
        ((self.header[0] as u16) << 8) | (self.header[1] as u16)
    }

    /// qr returns the query type.
    /// RFC 6762: 18.2. QR (Query/Response) Bit
    /// In query messages the QR bit MUST be zero. In response messages the QR bit MUST be one.
    pub fn qr(&self) -> QR {
        if (self.header[2] & 0x80) == 0 {
            return QR::Query;
        }
        QR::Response
    }

    /// is_query returns true if the message is a query.
    pub fn is_query(&self) -> bool {
        self.qr() == QR::Query
    }

    /// is_response returns true if the message is a response.
    pub fn is_response(&self) -> bool {
        self.qr() == QR::Response
    }

    /// Opcode returns the kind of query.
    /// RFC 6762: 18.3. OPCODE
    /// In both multicast query and multicast response messages, the OPCODE MUST be zero on transmission (only standard queries are currently supported over multicast).
    pub fn opcode(&self) -> Opcode {
        let opcode = ((self.header[2] & 0x78) >> 3) & 0x0F;
        match opcode {
            0 => Opcode::Query,
            1 => Opcode::IQuery,
            2 => Opcode::Status,
            _ => Opcode::Query,
        }
    }

    /// aa returns the authoritative answer bit.
    /// RFC 6762: 18.4. AA (Authoritative Answer) Bit
    /// In query messages, the Authoritative Answer bit MUST be zero on transmission, and MUST be ignored on reception.
    /// In response messages for Multicast domains, the Authoritative Answer bit MUST be set to one (not setting this bit would imply there's some other place where "better" information may be found) and MUST be ignored on reception.
    pub fn aa(&self) -> bool {
        (self.header[2] & 0x04) == 0x04
    }

    /// tc returns the truncated bit.
    /// RFC 6762: 18.5. TC (Truncated) Bit
    /// In query messages, if the TC bit is set, it means that additional Known-Answer records may be following shortly. A responder SHOULD record this fact, and wait for those additional Known-Answer records, before deciding whether to respond. If the TC bit is clear, it means that the querying host has no additional Known Answers.
    /// In multicast response messages, the TC bit MUST be zero on transmission, and MUST be ignored on reception.
    pub fn tc(&self) -> bool {
        (self.header[2] & 0x02) == 0x02
    }

    /// rd returns the recursion desired bit.
    /// RFC 6762: 18.6. RD (Recursion Desired) Bit
    /// In both multicast query and multicast response messages, the Recursion Desired bit SHOULD be zero on transmission, and MUST be ignored on reception.
    pub fn rd(&self) -> bool {
        (self.header[2] & 0x01) == 0x01
    }

    /// ra returns the recursion available bit.
    /// RFC 6762: 18.7. RA (Recursion Available) Bit
    /// In both multicast query and multicast response messages, the Recursion Available bit MUST be zero on transmission, and MUST be ignored on reception.
    pub fn ra(&self) -> bool {
        (self.header[3] & 0x80) == 0x80
    }

    /// z returns the zero bit.
    /// RFC 6762: 18.8. Z (Zero) Bit
    /// In both query and response messages, the Zero bit MUST be zero on transmission, and MUST be ignored on reception.
    pub fn z(&self) -> bool {
        (self.header[3] & 0x40) == 0x40
    }

    /// ad returns the authentic data bit.
    /// RFC 6762: 18.9. AD (Authentic Data) Bit
    /// In both multicast query and multicast response messages, the Authentic Data bit [RFC2535] MUST be zero on transmission, and MUST be ignored on reception.
    pub fn ad(&self) -> bool {
        (self.header[3] & 0x20) == 0x20
    }

    /// cd returns the checking disabled bit.
    /// RFC 6762: 18.10. CD (Checking Disabled) Bit
    /// In both multicast query and multicast response messages, the Checking Disabled bit [RFC2535] MUST be zero on transmission, and MUST be ignored on reception.
    pub fn cd(&self) -> bool {
        (self.header[3] & 0x10) == 0x10
    }

    /// response_code returns the checking disabled bit.
    /// RFC 6762: 18.11. RCODE (Response Code)
    /// In both multicast query and multicast response messages, the Response Code MUST be zero on transmission. Multicast DNS messages received with non-zero Response Codes MUST be silently ignored.
    pub fn response_code(&self) -> ResponseCode {
        let rcode = self.header[3] & 0x0F;
        match rcode {
            0 => ResponseCode::NoError,
            1 => ResponseCode::FormatError,
            2 => ResponseCode::ServerFailure,
            3 => ResponseCode::NameError,
            4 => ResponseCode::NotImplemented,
            5 => ResponseCode::Refused,
            _ => ResponseCode::NoError,
        }
    }

    fn set_number_of_entries(&mut self, offset: usize, num: u16) {
        self.header[offset] = ((num >> 8) & 0xFF) as u8;
        self.header[offset + 1] = (num & 0xFF) as u8;
    }

    fn number_of_entries(&self, offset: usize) -> u16 {
        ((self.header[offset] as u16) << 8) | (self.header[offset + 1] as u16)
    }

    /// set_qd_count sets the specified number to the QD field.
    pub fn set_qd_count(&mut self, num: u16) {
        self.set_number_of_entries(4, num)
    }

    /// qd_count returns the number of entries in the question section.
    pub fn qd_count(&self) -> u16 {
        self.number_of_entries(4)
    }

    /// set_an_count sets the specified number to the AN field.
    pub fn set_an_count(&mut self, num: u16) {
        self.set_number_of_entries(6, num)
    }

    /// an_count returns the number of entries in the answer section.
    pub fn an_count(&self) -> u16 {
        self.number_of_entries(6)
    }

    /// set_ns_count sets the specified number to the NS field.
    pub fn set_ns_count(&mut self, num: u16) {
        self.set_number_of_entries(8, num)
    }

    /// ns_count returns the number of entries in the authority section.
    pub fn ns_count(&self) -> u16 {
        self.number_of_entries(8)
    }

    /// set_ar_count sets the specified number to the AR field.
    pub fn set_ar_count(&mut self, num: u16) {
        self.set_number_of_entries(10, num)
    }

    /// ar_count returns the number of entries in the additional section.
    pub fn ar_count(&self) -> u16 {
        self.number_of_entries(10)
    }

    /// parse_bytes parses the specified message bytes.
    pub fn parse_bytes(&mut self, msg_bytes: &[u8]) -> Result<(), Error> {
        let mut reader = Reader::from_bytes(msg_bytes);

        // Header
        if reader.read_bytes(&mut self.header).is_err() {
            return Err(Error::new(msg_bytes, 0));
        }

        // Questions
        let qd_count = self.qd_count();
        for _ in 0..qd_count {
            let mut question = Record::new();
            question.parse_request_record(&mut reader)?;
            self.questions.push(question);
        }

        // Answers
        let an_count = self.an_count();
        for _ in 0..an_count {
            let mut answer = Record::new();
            answer.parse_resource_record(&mut reader)?;
            self.answers.push(answer);
        }

        // Authorities
        let ns_count = self.ns_count();
        for _ in 0..ns_count {
            let mut authority = Record::new();
            authority.parse_resource_record(&mut reader)?;
            self.authorities.push(authority);
        }

        // Additionals
        let ar_count = self.ar_count();
        for _ in 0..ar_count {
            let mut additional = Record::new();
            additional.parse_resource_record(&mut reader)?;
            self.additionals.push(additional);
        }

        Ok(())
    }

    /// questions returns the questions.
    pub fn questions(&self) -> &Vec<Record> {
        &self.questions
    }

    /// answers returns the answers.
    pub fn answers(&self) -> &Vec<Record> {
        &self.answers
    }

    /// authorities returns the authorities.
    pub fn authorities(&self) -> &Vec<Record> {
        &self.authorities
    }

    /// additionals returns the additionals.
    pub fn additionals(&self) -> &Vec<Record> {
        &self.additionals
    }

    /// find_record returns the record of the specified name.
    pub fn find_record(&self, name: &str) -> Option<&Record> {
        for question in self.questions() {
            if question.name() == name {
                return Some(question);
            }
        }
        for answer in self.answers() {
            if answer.name() == name {
                return Some(answer);
            }
        }
        for authority in self.authorities() {
            if authority.name() == name {
                return Some(authority);
            }
        }
        for additional in self.additionals() {
            if additional.name() == name {
                return Some(additional);
            }
        }
        None
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
        msg.parse_bytes(&self.bytes());
        msg
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}
