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

use crate::dns::a_record::ARecord;
use crate::dns::aaaa_record::AAAARecord;
use crate::dns::error::Error;
use crate::dns::nsec_record::NSECRecord;
use crate::dns::ptr_record::PTRRecord;
use crate::dns::record::Record;
use crate::dns::srv_record::SRVRecord;
use crate::dns::txt_record::TXTRecord;
use crate::dns::typ::Type;

/// ResourceRecord represents a DNS resource record.
pub trait ResourceRecord: Send {
    /// name returns the name of the record.
    fn name(&self) -> &str;
    /// typ returns the type of the record.
    fn typ(&self) -> Type;
    /// content returns the string representation of the record data.
    fn content(&self) -> &str;
}

impl Record {
    pub fn to_resource_record(&self) -> Result<Box<dyn ResourceRecord>, Error> {
        match self.typ() {
            Type::A => Ok(Box::new(ARecord::from_record(self)?)),
            Type::AAAA => Ok(Box::new(AAAARecord::from_record(self)?)),
            Type::TXT => Ok(Box::new(TXTRecord::from_record(self)?)),
            Type::SRV => Ok(Box::new(SRVRecord::from_record(self)?)),
            Type::PTR => Ok(Box::new(PTRRecord::from_record(self)?)),
            Type::NSEC => Ok(Box::new(NSECRecord::from_record(self)?)),
            _ => Err(Error::from_str(&format!(
                "Unsupported record type: {:?}",
                self.typ().to_string()
            ))),
        }
    }
}
