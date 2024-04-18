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
use crate::dns::record::Record;
use crate::dns::resource_record::ResourceRecord;
use crate::dns::typ::Type;
use std::fmt;

/// NSECRecord represents a NSEC record.
pub struct NSECRecord {
    name: String,
}

impl NSECRecord {
    /// from_record creates a new NSEC record from the specified record.
    pub fn from_record(record: &Record) -> Result<NSECRecord, Error> {
        Ok(NSECRecord {
            name: record.name().to_string(),
        })
    }

    /// typ returns the type of the record.
    pub fn typ(&self) -> Type {
        Type::NSEC
    }
}

impl ResourceRecord for NSECRecord {
    fn name(&self) -> &str {
        &self.name
    }

    fn typ(&self) -> Type {
        Type::NSEC
    }

    fn content(&self) -> &str {
        ""
    }
}

impl fmt::Display for NSECRecord {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}
