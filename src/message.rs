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

use crate::dns::{Error, Message};
use crate::query::Query;

/// QueryMessage represents a DNS-SD query message.
pub struct QueryMessage {
    msg: Message,
}

impl QueryMessage {
    /// Create a new query message.
    pub fn new(q: &Query) -> QueryMessage {
        QueryMessage {
            msg: Message::new(),
        }
    }

    /// set_id sets the ID of the query message.
    pub fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        self.msg.bytes()
    }
}
