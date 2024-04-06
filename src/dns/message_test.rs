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
        struct Test {
            // log: &'static str,
            log: Vec<u8>,
        }

        let tests = vec![Test {
            // log: include_str!("log/matter01.log"),
            log: include_bytes!("log/matter01.bin").to_vec(),
        }];

        for test in tests {
            let mut msg = Message::new();
            assert!(msg.parse_bytes(&test.log).is_ok());
        }
    }
}
