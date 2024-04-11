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

use std::any::Any;
use std::fmt;

use crate::dns::class::*;
use crate::dns::error::Error;
use crate::dns::reader::Reader;
use crate::dns::typ::*;

/// A structure representing a DNS record.
pub struct Record {
    name: String,
    data: Vec<u8>,
    typ: Type,
    class: Class,
    unicastResponse: bool,
}

/// A structure representing a DNS record.
impl Record {
    /// Create a new record.
    pub fn new() -> Record {
        Record {
            name: String::new(),
            data: Vec::new(),
            typ: Type::NONE,
            class: Class::NONE,
            unicastResponse: false,
        }
    }

    pub fn parse_request_record(&mut self, reader: &mut Reader) -> Result<(), Error> {
        self.parse_section(reader)?;
        Ok(())
    }

    pub fn parse_resource_record(&mut self, reader: &mut Reader) -> Result<(), Error> {
        self.parse_section(reader)?;
        Ok(())
    }

    fn parse_section(&mut self, reader: &mut Reader) -> Result<(), Error> {
        // Parse domain name.
        self.name = reader.read_name()?;

        // Parse type.
        self.typ = Type::from_value(reader.read_u16()?);

        // Parse class.
        let cls = reader.read_u16()?;
        self.class = Class::from_value(cls & CLASS_MASK);
        self.unicastResponse = (cls & UNICAST_RESPONSE_MASK) != 0;

        Ok(())
    }
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}
