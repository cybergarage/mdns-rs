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

use crate::dns::{Message, QuestionRecord, Result};
use crate::query::Query;

/// QueryMessage represents a DNS-SD query message.
pub struct QueryMessage {}

impl QueryMessage {
    /// Create a new query message.
    pub fn new(q: &Query) -> Message {
        let mut msg = Message::new();
        let mut qr = QuestionRecord::new();
        qr.set_name(&q.to_string());
        msg.add_question(qr);
        msg
    }
}
