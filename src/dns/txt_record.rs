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
use std::collections::HashMap;
use std::fmt;

pub struct TXTRecord {
    strs: Vec<String>,
    attrs: HashMap<String, String>,
}

impl TXTRecord {
    /// from_record creates a new TXT record from the specified record.
    pub fn from_record(record: &Record) -> Result<TXTRecord, Error> {
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
            strs: strs,
            attrs: attrs,
        };
        Ok(txt)
    }

    /// strings returns the strings of the TXT record.
    pub fn strings(&self) -> &Vec<String> {
        &self.strs
    }

    /// attributes returns the attributes of the TXT record.
    pub fn attributes(&self) -> &HashMap<String, String> {
        &self.attrs
    }
}

impl fmt::Display for TXTRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}
