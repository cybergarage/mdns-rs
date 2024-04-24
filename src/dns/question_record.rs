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

use crate::dns::class::Class;
use crate::dns::record::Record;
use crate::dns::typ::Type;

// QuestionRecord represents a question record.
pub struct QuestionRecord {}

impl QuestionRecord {
    /// Create a new question record.
    pub fn new() -> Record {
        let mut record = Record::new();
        record.set_typ(Type::PTR);
        record.set_class(Class::IN);
        record
    }
}
