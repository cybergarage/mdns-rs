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

    use crate::dns::message::Message;
    // use cybergarage::log::hexdump::*;

    #[test]
    fn message_parse() {
        struct Expected {
            qd_count: u16,
            an_count: u16,
            ns_count: u16,
            ar_count: u16,
        }
        struct Test {
            // log: &'static str,
            log: Vec<u8>,
            expected: Expected,
        }

        let tests = vec![Test {
            // log: include_str!("log/matter01.log"),
            log: include_bytes!("log/matter01.bin").to_vec(),
            expected: Expected {
                qd_count: 0,
                an_count: 5,
                ns_count: 0,
                ar_count: 0,
            },
        }];

        for test in tests {
            let mut msg = Message::new();
            assert!(msg.parse_bytes(&test.log).is_ok());
            assert_eq!(msg.qd_count(), test.expected.qd_count);
            assert_eq!(msg.an_count(), test.expected.an_count);
            assert_eq!(msg.ns_count(), test.expected.ns_count);
            assert_eq!(msg.ar_count(), test.expected.ar_count);
        }
    }
}
