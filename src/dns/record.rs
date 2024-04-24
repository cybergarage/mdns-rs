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

use crate::dns::class::*;
use crate::dns::error::Result;
use crate::dns::reader::Reader;
use crate::dns::typ::*;

/// A structure representing a DNS record.
pub struct Record {
    name: String,
    data: Vec<u8>,
    typ: Type,
    cls: Class,
    unicast_response: bool,
    ttl: u32,
}

impl Record {
    /// Create a new record.
    pub fn new() -> Record {
        Record {
            name: String::new(),
            data: Vec::new(),
            typ: Type::NONE,
            cls: Class::NONE,
            unicast_response: false,
            ttl: 0,
        }
    }

    /// set_name sets the name of the record.
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    /// name returns the name of the record.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// set_typ sets the type of the record.
    pub fn set_typ(&mut self, typ: Type) {
        self.typ = typ;
    }

    /// typ returns the type of the record.
    pub fn typ(&self) -> Type {
        self.typ
    }

    /// set_class sets the class of the record.
    pub fn set_class(&mut self, class: Class) {
        self.cls = class;
    }

    /// class returns the class of the record.
    pub fn class(&self) -> Class {
        self.cls
    }

    /// set_data sets the data of the record.
    pub fn set_data(&mut self, data: Vec<u8>) {
        self.data = data;
    }

    /// data returns the data of the record.
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// set_unicast_response sets the unicast response flag of the record.
    pub fn set_unicast_response(&mut self, unicast_response: bool) {
        self.unicast_response = unicast_response;
    }

    /// uniast_response returns the unicast response flag of the record.
    pub fn unicast_response(&self) -> bool {
        self.unicast_response
    }

    /// set_ttl sets the TTL of the record.
    pub fn set_ttl(&mut self, ttl: u32) {
        self.ttl = ttl;
    }

    /// ttl returns the TTL of the record.
    pub fn ttl(&self) -> u32 {
        self.ttl
    }

    /// parse_request_record parses a request record.
    pub fn parse_request_record(&mut self, reader: &mut Reader) -> Result<()> {
        self.parse_section(reader)?;
        Ok(())
    }

    /// parse_resource_record parses a resource record.
    pub fn parse_resource_record(&mut self, reader: &mut Reader) -> Result<()> {
        self.parse_section(reader)?;

        // Parse TTL.
        self.ttl = reader.read_u32()?;

        // Parse data length.
        let data_len = reader.read_u16()?;
        if 0 < data_len {
            let mut data = vec![0; data_len as usize];
            reader.read_bytes(&mut data)?;
            self.data = data;
        }

        Ok(())
    }

    fn parse_section(&mut self, reader: &mut Reader) -> Result<()> {
        // Parse domain name.
        self.name = reader.read_name()?;

        // Parse type.
        self.typ = Type::from_value(reader.read_u16()?);

        // Parse class.
        let cls = reader.read_u16()?;
        self.cls = Class::from_value(cls & CLASS_MASK);
        self.unicast_response = (cls & UNICAST_RESPONSE_MASK) != 0;

        Ok(())
    }
}

impl fmt::Display for Record {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}
