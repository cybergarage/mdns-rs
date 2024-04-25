// Copyright (C) 2022 Satoshi Konno All rights reserved.
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

#[cfg(test)]
mod tests {

    use crate::dns::class::Class;
    use crate::dns::message::Message;
    use crate::dns::typ::Type;
    use crate::message::QueryMessage;
    use crate::query::Query;

    #[test]
    fn query_message() {
        struct Expected {
            name: String,
        }
        struct Test {
            query: Query,
            expected: Expected,
        }

        let tests = vec![Test {
            query: Query::with("_http._tcp", "local"),
            expected: Expected {
                name: "_http._tcp.local".to_string(),
            },
        }];

        for test in tests {
            let query = QueryMessage::new(&test.query);
            let query_bytes = query.to_bytes();
            assert!(query_bytes.is_ok());
            let query_bytes = query_bytes.unwrap();
            let msg = Message::from_bytes(&query_bytes);
            assert!(msg.is_ok());
            let msg = msg.unwrap();
            assert_eq!(msg.qd_count(), 1);
            let question = &msg.questions()[0];
            assert_eq!(question.name(), test.expected.name);
            assert_eq!(question.typ(), Type::PTR);
            assert_eq!(question.class(), Class::IN);
        }
    }
}
