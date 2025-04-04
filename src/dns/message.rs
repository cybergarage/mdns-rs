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

use crate::dns::error::{Error, Result};
use crate::dns::reader::Reader;
use crate::dns::record::Record;
use crate::dns::records::Records;
use crate::dns::resource_record::*;
use crate::dns::resource_records::ResourceRecords;
use crate::dns::writer::Writer;

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
    questions: Records,
    answers: Records,
    authorities: Records,
    additionals: Records,
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
    pub fn from_bytes(msg_bytes: &[u8]) -> Result<Message> {
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

    /// set_id sets the query identifier.
    pub fn set_id(&mut self, id: u16) {
        self.header[0] = ((id >> 8) & 0xFF) as u8;
        self.header[1] = (id & 0xFF) as u8;
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

    /// set_qr sets the query type.
    pub fn set_qr(&mut self, qr: QR) {
        match qr {
            QR::Query => self.header[2] &= 0x7F,
            QR::Response => self.header[2] |= 0x80,
        }
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

    /// add_question adds the specified question.
    pub fn add_question(&mut self, question: Record) {
        self.questions.push(question);
        self.set_qd_count(self.questions.len() as u16);
    }

    /// questions returns the questions.
    pub fn questions(&self) -> &Records {
        &self.questions
    }

    /// add_answer adds the specified answer.
    pub fn add_answer(&mut self, answer: Record) {
        self.answers.push(answer);
        self.set_an_count(self.answers.len() as u16);
    }

    /// answers returns the answers.
    pub fn answers(&self) -> &Records {
        &self.answers
    }

    /// add_authority adds the specified authority.
    pub fn add_authority(&mut self, authority: Record) {
        self.authorities.push(authority);
        self.set_ns_count(self.authorities.len() as u16);
    }

    /// authorities returns the authorities.
    pub fn authorities(&self) -> &Records {
        &self.authorities
    }

    /// add_additional adds the specified additional.
    pub fn add_additional(&mut self, additional: Record) {
        self.additionals.push(additional);
        self.set_ar_count(self.additionals.len() as u16);
    }

    /// additionals returns the additionals.
    pub fn additionals(&self) -> &Records {
        &self.additionals
    }

    /// resource_records returns the all resource records.
    pub fn resource_records(&self) -> ResourceRecords {
        let mut resouce_records: Vec<Box<dyn ResourceRecord>> = Vec::new();
        for answer in self.answers() {
            match answer.to_resource_record() {
                Ok(resource_record) => resouce_records.push(resource_record),
                Err(_) => {}
            }
        }
        for authority in self.authorities() {
            match authority.to_resource_record() {
                Ok(resource_record) => resouce_records.push(resource_record),
                Err(_) => {}
            }
        }
        for additional in self.additionals() {
            match additional.to_resource_record() {
                Ok(resource_record) => resouce_records.push(resource_record),
                Err(_) => {}
            }
        }
        resouce_records
    }

    /// parse_bytes parses the specified message bytes.
    pub fn parse_bytes(&mut self, msg_bytes: &[u8]) -> Result<()> {
        let mut reader = Reader::from_bytes(msg_bytes);

        // Header
        if reader.read_bytes(&mut self.header).is_err() {
            return Err(Error::from_bytes(msg_bytes, 0));
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

    /// to_string returns the message as a string.
    pub fn to_string(&self) -> String {
        let mut msg_str = String::new();
        struct Record(String, String, String);
        let mut records = Vec::new();
        for rrecord in self.resource_records() {
            let record = Record(
                rrecord.name().to_string(),
                rrecord.typ().to_string(),
                rrecord.content().to_string(),
            );
            records.push(record);
        }
        for record in records {
            msg_str.push_str(&format!("{} {} {}\n", record.0, record.1, record.2));
        }
        msg_str
    }

    /// to_bytes returns the message as bytes.
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut w = Writer::new();
        w.write_bytes(&self.header)?;
        for question in self.questions() {
            w.write_request_record(&question)?;
        }
        for answer in self.answers() {
            w.write_response_record(&answer)?;
        }
        for authority in self.authorities() {
            w.write_response_record(&authority)?;
        }
        for additional in self.additionals() {
            w.write_response_record(&additional)?;
        }
        Ok(w.to_bytes())
    }
}

impl Clone for Message {
    fn clone(&self) -> Message {
        let mut msg = Message::new();
        match msg.to_bytes() {
            Ok(bytes) => {
                let _ = msg.parse_bytes(&bytes);
            }
            Err(_) => {}
        }
        msg
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
