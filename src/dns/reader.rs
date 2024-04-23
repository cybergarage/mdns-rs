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

pub struct Reader<'a> {
    buffer: &'a [u8],
    buffer_len: usize,
    cursor: usize,
}

/// A structure representing a DNS reader.
impl<'a> Reader<'a> {
    /// from_bytes creates a new reader from the specified bytes.
    pub fn from_bytes(msg_bytes: &'a [u8]) -> Reader<'a> {
        Reader {
            buffer: msg_bytes,
            buffer_len: msg_bytes.len(),
            cursor: 0,
        }
    }

    /// set_offset sets the offset of the buffer.
    pub fn set_offset(&mut self, offset: usize) {
        self.cursor = offset;
    }

    /// offset returns the offset of the buffer.
    pub fn offset(&self) -> usize {
        self.cursor
    }

    // read_u8 reads the next byte from the buffer.
    pub fn read_u8(&mut self) -> Result<u8, Error> {
        if self.buffer_len < self.cursor {
            return Err(Error::from_bytes(self.buffer, self.cursor));
        }
        let v = self.buffer[self.cursor];
        self.cursor += 1;
        Ok(v)
    }

    /// read_u16 reads the next 16-bit integer from the buffer.
    pub fn read_u16(&mut self) -> Result<u16, Error> {
        let mut buf = [0; 2];
        self.read_bytes(&mut buf)?;
        Ok(u16::from_be_bytes(buf))
    }

    /// read_u32 reads the next 32-bit integer from the buffer.
    pub fn read_u32(&mut self) -> Result<u32, Error> {
        let mut buf = [0; 4];
        self.read_bytes(&mut buf)?;
        Ok(u32::from_be_bytes(buf))
    }

    /// read_bytes reads the next bytes into the buffer.
    pub fn read_bytes(&mut self, buf: &mut [u8]) -> Result<(), Error> {
        if self.buffer_len < self.cursor + buf.len() {
            return Err(Error::from_bytes(self.buffer, self.cursor));
        }
        buf.copy_from_slice(&self.buffer[self.cursor..self.cursor + buf.len()]);
        self.cursor += buf.len();
        Ok(())
    }

    /// read_string_size reads the next string size from the buffer.
    pub fn read_string_size(&mut self) -> Result<usize, Error> {
        if self.buffer_len < self.cursor {
            return Err(Error::from_bytes(self.buffer, self.cursor));
        }
        let str_len = self.buffer[self.cursor] as usize;
        self.cursor += 1;
        Ok(str_len)
    }

    /// read_string reads the next string from the buffer.
    pub fn read_string(&mut self) -> Result<String, Error> {
        let str_len = self.read_string_size()?;
        if self.buffer_len < self.cursor + str_len {
            return Err(Error::from_bytes(self.buffer, self.cursor));
        }
        let str_bytes = &self.buffer[self.cursor..self.cursor + str_len];
        self.cursor += str_len;
        Ok(String::from_utf8(str_bytes.to_vec()).unwrap())
    }

    pub fn read_strings(&mut self) -> Result<Vec<String>, Error> {
        let mut strs = Vec::new();
        loop {
            let str_len = self.read_string_size()?;
            if str_len == 0 {
                break;
            }
            if self.buffer_len < self.cursor + str_len {
                return Err(Error::from_bytes(self.buffer, self.cursor));
            }
            let str_bytes = &self.buffer[self.cursor..self.cursor + str_len];
            self.cursor += str_len;
            strs.push(String::from_utf8(str_bytes.to_vec()).unwrap());
        }
        Ok(strs)
    }

    /// read_name reads the next name from the buffer.
    pub fn read_name(&mut self) -> Result<String, Error> {
        let mut name = String::new();
        let mut is_compressed = false;
        loop {
            let label_len = self.buffer[self.cursor] as usize;
            if label_len == 0 {
                self.cursor += 1;
                break;
            }
            if label_len & 0xc0 == 0xc0 {
                if !is_compressed {
                    is_compressed = true;
                }
                let offset =
                    ((label_len as usize) & 0x3f) << 8 | self.buffer[self.cursor + 1] as usize;
                self.cursor += 2;
                let mut reader = Reader::from_bytes(&self.buffer[offset..]);
                let compressed_name = reader.read_name()?;
                if 0 < name.len() {
                    name.push('.');
                }
                name.push_str(&compressed_name);
                break;
            }
            self.cursor += 1;
            if 0 < name.len() {
                name.push('.');
            }
            let label_bytes = &self.buffer[self.cursor..self.cursor + label_len];
            name.push_str(&String::from_utf8(label_bytes.to_vec()).unwrap());
            self.cursor += label_len;
        }
        Ok(name)
    }
}
