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

use hex;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Error {
    pub message: String,
}

impl Error {
    /// from_str creates a new Error with the specified string.
    pub fn from_str(str: &str) -> Error {
        Error {
            message: str.to_string(),
        }
    }

    /// from_bytes creates a new Error with the specified bytes.
    pub fn from_bytes(msg_bytes: &[u8], offset: usize) -> Error {
        Error {
            message: format!(
                "Invalid bytes {} (offset:{})",
                hex::encode(msg_bytes),
                offset
            ),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for Error {}
