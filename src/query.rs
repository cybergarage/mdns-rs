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

/// Query represents a DNS-SD query.
pub struct Query {
    service: String,
    domain: String,
}

impl Query {
    /// new creates a new query.
    pub fn new() -> Query {
        Query {
            service: String::new(),
            domain: String::new(),
        }
    }

    /// with creates a new query with the specified service and domain.
    pub fn with(service: &str, domain: &str) -> Query {
        Query {
            service: service.to_string(),
            domain: domain.to_string(),
        }
    }

    /// set_service sets the service of the query.
    pub fn set_service(&mut self, service: &str) {
        self.service = service.to_string();
    }

    /// service returns the service of the query.
    pub fn service(&self) -> &str {
        &self.service
    }

    /// set_domain sets the domain of the query.
    pub fn set_domain(&mut self, domain: &str) {
        self.domain = domain.to_string();
    }

    /// domain returns the domain of the query.
    pub fn domain(&self) -> &str {
        &self.domain
    }

    /// to_string returns the string representation of the query.
    pub fn to_string(&self) -> String {
        format!("{}.{}", self.service, self.domain)
    }
}
