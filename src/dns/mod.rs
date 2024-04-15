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

pub use self::a_record::*;
pub use self::aaaa_record::*;
pub use self::class::*;
pub use self::error::*;
pub use self::message::*;
pub use self::nsec_record::*;
pub use self::ptr_record::*;
pub use self::record::*;
pub use self::srv_record::*;
pub use self::txt_record::*;
pub use self::typ::*;

mod a_record;
mod aaaa_record;
mod class;
mod error;
mod message;
mod nsec_record;
mod ptr_record;
mod reader;
mod record;
mod srv_record;
mod txt_record;
mod typ;

mod message_test;
mod reader_test;
