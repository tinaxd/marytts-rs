// Copyright 2019 tinaxd
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate marytts_rs;

fn main() {
    let srv = marytts_rs::TTSServer::new("localhost", 59125);
    let query = marytts_rs::TTSQuery::default_text("The quick brown fox jumps over the lazy dog");
    let file = std::fs::File::create("test.wav").expect("Failed to create a new file");

    let mut bufw = std::io::BufWriter::new(file);
    marytts_rs::tts_blocking(&srv, &query, &mut bufw).unwrap();
}
