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

pub const TYPE_MASK: u16 = 0x7fff;

#[derive(PartialEq)]
pub enum Type {
    NONE = 0x0000,
    A = 0x0001,
    NS = 0x0002,
    CNAME = 0x0005,
    SOA = 0x0006,
    PTR = 0x000c,
    MX = 0x000f,
    TXT = 0x0010,
    AAAA = 0x001c,
    SRV = 0x0021,
    NAPTR = 0x0023,
    OPT = 0x0029,
    ANY = 0x00ff,
}

impl Type {
    pub fn from_value(value: u16) -> Type {
        match value {
            0x0001 => Type::A,
            0x0002 => Type::NS,
            0x0005 => Type::CNAME,
            0x0006 => Type::SOA,
            0x000c => Type::PTR,
            0x000f => Type::MX,
            0x0010 => Type::TXT,
            0x001c => Type::AAAA,
            0x0021 => Type::SRV,
            0x0023 => Type::NAPTR,
            0x0029 => Type::OPT,
            0x00ff => Type::ANY,
            _ => Type::NONE,
        }
    }

    pub fn to_value(&self) -> u16 {
        match self {
            Type::A => 0x0001,
            Type::NS => 0x0002,
            Type::CNAME => 0x0005,
            Type::SOA => 0x0006,
            Type::PTR => 0x000c,
            Type::MX => 0x000f,
            Type::TXT => 0x0010,
            Type::AAAA => 0x001c,
            Type::SRV => 0x0021,
            Type::NAPTR => 0x0023,
            Type::OPT => 0x0029,
            Type::ANY => 0x00ff,
            Type::NONE => 0x0000,
        }
    }
}
