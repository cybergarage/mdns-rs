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

pub struct TXTRecord {
    strs: Vec<String>,
}

impl TXTRecord {
    pub fn from_record(record: &Record) -> Result<TXTRecord, Error> {
        let data = record.data();
        let mut reader = Reader::from_bytes(data);
        let strs = reader.read_strings()?;
        let txt = TXTRecord { strs: strs };
        Ok(txt)
    }

    pub fn strings(&self) -> &Vec<String> {
        &self.strs
    }
}

impl fmt::Display for TXTRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}
