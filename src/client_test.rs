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

#[cfg(test)]
mod tests {

    use cybergarage::log::Logger;

    use crate::{Client, Query};

    #[test]
    fn client() {
        Logger::init();

        let mut client = Client::new();
        let ret = client.start();
        assert!(ret.is_ok(), "{:?}", ret);
        let queries = vec![Query::with("_services._dns-sd._udp", "local")];
        for query in &queries {
            let ret = client.search(query);
            assert!(ret.is_ok(), "{:?}", ret);
        }
        assert!(client.stop().is_ok());
    }
}
