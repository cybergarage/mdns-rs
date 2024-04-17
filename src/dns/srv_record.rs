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

use crate::dns::error::Error;
use crate::dns::reader::Reader;
use crate::dns::record::Record;
use std::fmt;

pub struct SRVRecord {
    service: String,
    proto: String,
    name: String,
    priority: u16,
    weight: u16,
    port: u16,
    target: String,
}

impl SRVRecord {
    /// from_record creates a new SRV record from the specified record.
    pub fn from_record(record: &Record) -> Result<SRVRecord, Error> {
        let mut srv = SRVRecord {
            service: "".to_string(),
            proto: "".to_string(),
            name: "".to_string(),
            priority: 0,
            weight: 0,
            port: 0,
            target: "".to_string(),
        };
        let data = record.data();
        if data.len() == 0 {
            return Ok(srv);
        }
        let mut reader = Reader::from_bytes(data);
        srv.priority = reader.read_u16()?;
        srv.weight = reader.read_u16()?;
        srv.port = reader.read_u16()?;
        srv.target = reader.read_name()?;
        Ok(srv)
    }

    /// service returns the service of the SRV record.
    pub fn service(&self) -> &str {
        &self.service
    }

    /// proto returns the protocol of the SRV record.
    pub fn proto(&self) -> &str {
        &self.proto
    }

    /// name returns the name of the SRV record.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// priority returns the priority of the SRV record.
    pub fn priority(&self) -> u16 {
        self.priority
    }

    /// weight returns the weight of the SRV record.
    pub fn weight(&self) -> u16 {
        self.weight
    }

    /// port returns the port of the SRV record.
    pub fn port(&self) -> u16 {
        self.port
    }

    /// target returns the target of the SRV record.
    pub fn target(&self) -> &str {
        &self.target
    }
}

impl fmt::Display for SRVRecord {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}
