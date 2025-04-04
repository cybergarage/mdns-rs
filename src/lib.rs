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

pub use self::client::Client;
pub use self::discoverer::Discoverer;
pub use self::error::{Error, Result};
pub use self::query::Query;
pub use self::service::Service;

pub mod client;
pub mod default;
pub mod discoverer;
pub mod dns;
pub mod error;
pub mod message;
pub mod query;
pub mod service;

mod client_test;
mod message_test;
