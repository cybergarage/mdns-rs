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

use crate::dns::error::Result;
use crate::dns::reader::Reader;
use crate::dns::record::Record;
use crate::dns::resource_record::ResourceRecord;
use crate::dns::typ::Type;
use std::fmt;

/// PTRRecord represents a PTR record.
pub struct PTRRecord {
    name: String,

    domain_name: String,
}

impl PTRRecord {
    /// from_record creates a new PTR record from the specified record.
    pub fn from_record(record: &Record) -> Result<PTRRecord> {
        let data = record.data();
        let mut reader = Reader::from_bytes(data);
        let domain_name = reader.read_name()?;
        let ptr = PTRRecord {
            name: record.name().to_string(),
            domain_name: domain_name.to_string(),
        };
        Ok(ptr)
    }

    ///
    pub fn domain_name(&self) -> &str {
        &self.domain_name
    }
}

impl ResourceRecord for PTRRecord {
    fn name(&self) -> &str {
        &self.name
    }

    fn typ(&self) -> Type {
        Type::PTR
    }

    fn content(&self) -> &str {
        ""
    }
}

impl fmt::Display for PTRRecord {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}
