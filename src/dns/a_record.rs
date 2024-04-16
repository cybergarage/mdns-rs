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
use std::fmt;
use std::net::IpAddr;
pub struct ARecord {
    ipaddr: IpAddr,
}

impl ARecord {
    pub fn from(record: &Record) -> Result<ARecord, Error> {
        let data = record.data();
        let addr = if let Ok(arr) = data[0..16].try_into() {
            IpAddr::from(arr)
        } else {
            return Err(Error::new(data, 0));
        };
        let a = ARecord { ipaddr: addr };
        Ok(a)
    }
}

impl fmt::Display for ARecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}
