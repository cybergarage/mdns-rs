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

    use crate::dns::reader::Reader;

    #[test]
    fn reader_read_bytes() {
        let mut reader = Reader::new(&[0x01, 0x02, 0x03]);
        let mut buf = [0; 1];
        assert!(reader.read_bytes(&mut buf).is_ok());
        assert_eq!(buf, [0x01]);
        assert!(reader.read_bytes(&mut buf).is_ok());
        assert_eq!(buf, [0x02]);
        assert!(reader.read_bytes(&mut buf).is_ok());
        assert_eq!(buf, [0x03]);
        assert!(reader.read_bytes(&mut buf).is_err());
    }

    #[test]
    fn reader_read_string() {
        let mut reader = Reader::new(&[0x03, 'a' as u8, 'b' as u8, 'c' as u8]);
        assert_eq!(reader.read_string().unwrap(), "abc");
    }

    #[test]
    fn reader_read_name() {

        struct Test {
            data: Vec<u8>,
            name: String,
        }

        let tests = vec![
            Test {
                data: vec![0x03, 'a' as u8, 'b' as u8, 'c' as u8, 0x00],
                name: "abc".to_string(),
            },
            Test {
                data: vec![
                    0x03, 'a' as u8, 'b' as u8, 'c' as u8, 0x03, 'd' as u8, 'e' as u8, 'f' as u8,
                    0x00,
                ],
                name: "abc.def".to_string(),
            },
        ];

        for test in tests {
            let mut reader = Reader::new(&test.data);
            assert_eq!(reader.read_name().unwrap(), test.name);
        }
    }
}
