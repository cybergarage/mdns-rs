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

use cybergarage::log::Logger;
use mdns::Client;

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

    let client = Client::new();

    client.lock().unwrap().start();
    client.lock().unwrap().stop();

    Ok(())
}
