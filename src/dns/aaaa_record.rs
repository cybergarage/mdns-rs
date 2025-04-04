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

use crate::dns::error::{Error, Result};
use crate::dns::record::Record;
use crate::dns::resource_record::ResourceRecord;
use crate::dns::typ::Type;
use std::fmt;
use std::net::{IpAddr, Ipv6Addr};

/// AAAARecord represents an AAAA record.
pub struct AAAARecord {
    name: String,
    ipaddr: IpAddr,
}

impl AAAARecord {
    /// from_record creates a new AAAA record from the specified record.
    pub fn from_record(record: &Record) -> Result<AAAARecord> {
        let data = record.data();
        let addr = if data.len() >= 16 {
            let arr: [u8; 16] = data[0..16].try_into().unwrap();
            IpAddr::V6(Ipv6Addr::from(arr))
        } else {
            return Err(Error::from_bytes(data, 0));
        };
        let a = AAAARecord {
            name: record.name().to_string(),
            ipaddr: addr,
        };
        Ok(a)
    }

    /// typ returns the type of the AAAA record.
    pub fn typ(&self) -> Type {
        Type::AAAA
    }

    /// ipaddr returns the IP address of the AAAA record.
    pub fn ipaddr(&self) -> &IpAddr {
        &self.ipaddr
    }
}

impl ResourceRecord for AAAARecord {
    fn name(&self) -> &str {
        &self.name
    }

    fn typ(&self) -> Type {
        Type::AAAA
    }

    fn content(&self) -> &str {
        ""
    }
}

impl fmt::Display for AAAARecord {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}
