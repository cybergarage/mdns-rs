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

pub const UNICAST_RESPONSE_MASK: u16 = 0x8000;
pub const CACHE_FLUSH_MASK: u16 = 0x8000;
pub const CLASS_MASK: u16 = 0x7fff;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Class {
    NONE = 0x0000,
    IN = 0x0001,
}

impl Class {
    pub fn from_value(value: u16) -> Class {
        match value {
            0x0001 => Class::IN,
            _ => Class::NONE,
        }
    }

    pub fn to_value(&self) -> u16 {
        match self {
            Class::IN => 0x0001,
            _ => 0x0000,
        }
    }
}
