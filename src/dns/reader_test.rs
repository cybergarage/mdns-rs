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
}
