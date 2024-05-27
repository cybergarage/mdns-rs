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

use std::env;
use std::io::Error;
use std::{thread, time};

use cybergarage::log::Logger;
use mdns::{Client, Query};

fn usages() {
    println!("Usage: mdns-browse");
    println!(" -h : Print this message");
    println!(" -v : Enable debug output");
}

fn main() -> Result<(), Error> {
    for arg in env::args() {
        match arg.as_str() {
            "-v" => {
                Logger::init();
            }
            "-h" => {
                usages();
                return Ok(());
            }
            &_ => {}
        }
    }

    let mut client = Client::new();
    let ret = client.start();
    if ret.is_err() {
        return ret;
    }
    let queries = vec![Query::with("_services._dns-sd._udp", "local")];
    for query in &queries {
        client.search(query);
    }

    let ten_secs = time::Duration::from_secs(10);
    thread::sleep(ten_secs);

    let ret = client.stop();
    if ret.is_err() {
        return ret;
    }

    for service in client.services() {
        println!("Service : {}", service);
    }

    Ok(())
}
