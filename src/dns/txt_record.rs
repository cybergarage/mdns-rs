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
use std::collections::HashMap;
use std::fmt;

pub struct TXTRecord {
    name: String,
    strs: Vec<String>,
    attrs: HashMap<String, String>,
}

impl TXTRecord {
    /// from_record creates a new TXT record from the specified record.
    pub fn from_record(record: &Record) -> Result<TXTRecord> {
        let data = record.data();
        let mut reader = Reader::from_bytes(data);
        let strs = reader.read_strings()?;
        let mut attrs = HashMap::new();
        for s in &strs {
            let mut kv = s.splitn(2, '=');
            let key = kv.next().unwrap().to_string();
            let value = kv.next().unwrap_or("").to_string();
            attrs.insert(key, value);
        }
        let txt = TXTRecord {
            name: record.name().to_string(),
            strs: strs,
            attrs: attrs,
        };
        Ok(txt)
    }

    /// name returns the name of the TXT record.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// strings returns the strings of the TXT record.
    pub fn strings(&self) -> &Vec<String> {
        &self.strs
    }

    /// attributes returns the attributes of the TXT record.
    pub fn attributes(&self) -> &HashMap<String, String> {
        &self.attrs
    }

    /// attribute returns the attribute of the TXT record.
    pub fn attribute(&self, key: &str) -> Option<&String> {
        self.attrs.get(key)
    }
}

impl ResourceRecord for TXTRecord {
    fn name(&self) -> &str {
        &self.name
    }

    fn typ(&self) -> Type {
        Type::TXT
    }

    fn content(&self) -> &str {
        ""
    }
}

impl fmt::Display for TXTRecord {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}
