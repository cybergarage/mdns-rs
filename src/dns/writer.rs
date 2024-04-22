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

/// Writer represents a DNS writer.
pub struct Writer {
    buffer: Vec<u8>,
}

impl Writer {
    /// new creates a new writer.
    pub fn new() -> Writer {
        Writer { buffer: Vec::new() }
    }

    /// write_u8 writes a u8 value.
    pub fn write_u8(&mut self, value: u8) -> Result<(), Error> {
        self.buffer.push(value);
        Ok(())
    }
}
