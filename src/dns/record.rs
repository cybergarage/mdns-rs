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

/// A structure representing a DNS record.
pub struct Record {
    name: String,
    data: Vec<u8>,
}

/// A structure representing a DNS record.
impl Record {
    /// Create a new record.
    pub fn new() -> Record {
        Record {
            name: String::new(),
            data: Vec::new(),
        }
    }
    /// Create a new record from the specified bytes.
    pub fn from_bytes(msg_bytes: &[u8]) -> Record {
        let mut record = Record::new();
        record.data = msg_bytes.to_vec();
        record
    }

    pub fn from_reader(reader: &mut Reader) -> Result<Record, Error> {
        let mut record = Record::new();
        let res = record.parse_reader(reader);
        if res.is_err() {
            return Err(res.unwrap_err());
        }
        Ok(record)
    }

    fn parse_reader(&mut self, reader: &mut Reader) -> Result<(), Error> {
        self.parse_resouce(reader)?;
        Ok(())
    }

    fn parse_resouce(&mut self, reader: &mut Reader) -> Result<(), Error> {
        self.name = reader.read_name()?;
        Ok(())
    }
}

impl Clone for Record {
    fn clone(&self) -> Record {
        Record::from_bytes(&self.data)
    }
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}
