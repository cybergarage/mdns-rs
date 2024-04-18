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
use crate::dns::typ::Type;
// use create::dns::a_record::ARecord;
// use create::dns::aaaa_record::AAAARecord;

/// ResourceRecord represents a DNS resource record.
pub trait ResourceRecord: Send {
    // fn name(&self) -> &str;
    fn typ(&self) -> Type;
    // fn content(&self) -> &str;
}

pub fn resource_record_from_record(record: &Record) -> Result<Box<dyn ResourceRecord>, Error> {
    match record.typ() {
        // Type::A => Ok(ARecord::from_record(record)),
        // Type::AAAA => Ok(AAAARecord::from_record(record)),
        // Type::CNAME => Ok(CNAMERecord::from_record(record)),
        // Type::MX => Ok(MXRecord::from_record(record)),
        // Type::NSEC => Ok(NSECRecord::from_record(record)),
        // Type::NS => Ok(NSRecord::from_record(record)),
        // Type::PTR => Ok(PTRRecord::from_record(record)),
        // Type::SOA => Ok(SOARecord::from_record(record)),
        // Type::SRV => Ok(SRVRecord::from_record(record)),
        // Type::TXT => Ok(TXTRecord::from_record(record)),
        _ => Err(Error::from_bytes(record.data(), 0)),
    }
}
