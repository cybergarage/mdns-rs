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

use crate::dns::class::Class;
use crate::dns::class::UNICAST_RESPONSE_MASK;
use crate::dns::error::Error;
use crate::dns::record::Record;
use crate::dns::typ::Type;

/// Writer represents a DNS writer.
pub struct Writer {
    buffer: Vec<u8>,
}

impl Writer {
    /// new creates a new writer.
    pub fn new() -> Writer {
        Writer { buffer: Vec::new() }
    }

    /// write_u8 writes a u8 value.
    pub fn write_u8(&mut self, value: u8) -> Result<(), Error> {
        self.buffer.push(value);
        Ok(())
    }

    /// write_u16 writes a u16 value.
    pub fn write_u16(&mut self, value: u16) -> Result<(), Error> {
        self.buffer.push(((value >> 8) & 0xff) as u8);
        self.buffer.push((value & 0xff) as u8);
        Ok(())
    }

    /// write_u32 writes a u32 value.
    pub fn write_u32(&mut self, value: u32) -> Result<(), Error> {
        self.buffer.push(((value >> 24) & 0xff) as u8);
        self.buffer.push(((value >> 16) & 0xff) as u8);
        self.buffer.push(((value >> 8) & 0xff) as u8);
        self.buffer.push((value & 0xff) as u8);
        Ok(())
    }

    /// write_bytes writes a byte slice.
    pub fn write_bytes(&mut self, bytes: &[u8]) -> Result<(), Error> {
        for b in bytes {
            self.buffer.push(*b);
        }
        Ok(())
    }

    /// write_header writes a header.
    pub fn write_header(&mut self, header: &[u8]) -> Result<(), Error> {
        self.write_bytes(header)
    }

    /// write_type writes a type.
    pub fn write_type(&mut self, typ: Type) -> Result<(), Error> {
        self.write_u16(typ as u16)
    }

    /// write_class writes a class.
    pub fn write_class(&mut self, class: Class) -> Result<(), Error> {
        self.write_u16(class as u16)
    }

    /// write_ttl writes a TTL.
    pub fn write_ttl(&mut self, ttl: u32) -> Result<(), Error> {
        self.write_u32(ttl)
    }

    /// write_data writes data.
    pub fn write_data(&mut self, data: &[u8]) -> Result<(), Error> {
        let len = data.len();
        self.write_u16(len as u16)?;
        self.write_bytes(data)
    }

    /// write_name writes a domain name.
    pub fn write_name(&mut self, name: &str) -> Result<(), Error> {
        let labels = name.split('.');
        for label in labels {
            let len = label.len();
            self.write_u8(len as u8)?;
            for c in label.chars() {
                self.write_u8(c as u8)?;
            }
        }
        self.write_u8(0)?;
        Ok(())
    }

    /// write_request_record writes a request record.
    pub fn write_request_record(&mut self, record: &Record) -> Result<(), Error> {
        self.write_name(record.name())?;
        self.write_type(record.typ())?;
        let mut cls = record.class() as u16;
        if record.unicast_response() {
            cls |= UNICAST_RESPONSE_MASK;
        }
        self.write_u16(cls)?;
        Ok(())
    }

    /// write_response_record writes a response record.
    pub fn write_response_record(&mut self, record: &Record) -> Result<(), Error> {
        self.write_request_record(record)?;
        self.write_ttl(record.ttl())?;
        self.write_data(record.data())?;
        Ok(())
    }

    /// to_bytes returns the byte slice.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.buffer.clone()
    }
}
