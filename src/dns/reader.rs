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
use std::io::BufReader;
use std::io::Read;

use crate::dns::error::Error;

pub struct Reader<'a> {
    buffer: &'a [u8],
    buffer_len: usize,
    cursor: usize,
}

/// A structure representing a DNS reader.
impl<'a> Reader<'a> {
    /// Create a new reader from the specified bytes.
    pub fn new(msg_bytes: &'a [u8]) -> Reader<'a> {
        Reader {
            buffer: msg_bytes,
            buffer_len: msg_bytes.len(),
            cursor: 0,
        }
    }

    /// read_bytes reads the next bytes into the buffer.
    pub fn read_bytes(&mut self, buf: &mut [u8]) -> Result<(), Error> {
        if self.buffer_len < self.cursor + buf.len() {
            return Err(Error::new(self.buffer, self.cursor));
        }
        buf.copy_from_slice(&self.buffer[self.cursor..self.cursor + buf.len()]);
        self.cursor += buf.len();
        Ok(())
    }

    /// read_string_size reads the next string size.
    pub fn read_string_size(&mut self) -> Result<usize, Error> {
        if self.buffer_len < self.cursor {
            return Err(Error::new(self.buffer, self.cursor));
        }
        let str_len = self.buffer[self.cursor] as usize;
        self.cursor += 1;
        Ok(str_len)
    }

    /// read_string reads the next string.
    pub fn read_string(&mut self) -> Result<String, Error> {
        let str_len = self.read_string_size()?;
        if self.buffer_len < self.cursor + str_len {
            return Err(Error::new(self.buffer, self.cursor));
        }
        let str_bytes = &self.buffer[self.cursor..self.cursor + str_len];
        self.cursor += str_len;
        Ok(String::from_utf8(str_bytes.to_vec()).unwrap())
    }
}
