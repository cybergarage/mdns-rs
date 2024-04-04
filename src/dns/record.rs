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

use std::fmt;

/// A structure representing a DNS record.
pub struct Record {
    bytes: Vec<u8>,
}

/// A structure representing a DNS record.
impl Record {
    /// Create a new record.
    pub fn new() -> Record {
        Record { bytes: Vec::new() }
    }

    /// Create a new record from the specified bytes.
    pub fn from_bytes(msg_bytes: &[u8]) -> Record {
        let mut record = Record::new();
        record.bytes = msg_bytes.to_vec();
        record
    }

    /// Create a new record from the specified bytes.
    pub fn bytes(&self) -> Vec<u8> {
        self.bytes
    }
}

impl Clone for Record {
    fn clone(&self) -> Record {
        let mut r = Record::new();
        r
    }
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}
