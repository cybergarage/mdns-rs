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
pub use self::question_record::*;
pub use self::record::*;
pub use self::records::*;
pub use self::resource_record::*;
pub use self::resource_records::*;
pub use self::srv_record::*;
pub use self::txt_record::*;
pub use self::typ::*;
pub use self::writer::*;

pub mod a_record;
pub mod aaaa_record;
pub mod class;
pub mod error;
pub mod message;
pub mod nsec_record;
pub mod ptr_record;
pub mod question_record;
pub mod reader;
pub mod record;
pub mod records;
pub mod resource_record;
pub mod resource_records;
pub mod srv_record;
pub mod txt_record;
pub mod typ;
pub mod writer;

pub mod message_test;
pub mod reader_test;
