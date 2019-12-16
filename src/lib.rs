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

extern crate reqwest;
extern crate url;

#[derive(Debug, Clone)]
pub enum TTSError {
    ConnectionError(String)
}

impl From<reqwest::Error> for TTSError {
    fn from(t: reqwest::Error) -> TTSError {
        TTSError::ConnectionError(t.to_string())
    }
}

#[derive(Debug, Clone)]
pub struct TTSServer {
    host: String,
    port: u32
}

impl TTSServer {
    pub fn new(host: &str, port: u32) -> TTSServer {
        TTSServer {host: host.to_string(), port}
    }

    pub fn get_url(&self) -> String {
        format!("http://{}:{}", self.host, self.port)
    }
}

#[derive(Debug, Clone)]
pub struct TTSQuery {
    input_text: String,
    locale: String,
    voice: String,
}

impl TTSQuery {
    pub fn new(input_text: &str, locale: &str, voice: &str) -> TTSQuery {
        TTSQuery {
            input_text: input_text.to_string(),
            locale: locale.to_string(),
            voice: voice.to_string()
        }
    }

    pub fn default_text(input_text: &str) -> TTSQuery {
        Self::new(input_text, "en_US", "cmu-slt-hsmm")
    }

    #[allow(dead_code)]
    fn make_urlencode(&self) -> String {
        use url::form_urlencoded::Serializer;
        let mut encoded = Serializer::new(String::new());
        encoded.append_pair("INPUT_TEXT", &self.input_text);
        encoded.append_pair("LOCALE", &self.locale);
        encoded.append_pair("VOICE", &self.voice);
        encoded.finish()
    }

    fn make_hashmap(&self) -> std::collections::HashMap<&'static str, String> {
        let mut hashmap = std::collections::HashMap::new();
        hashmap.insert("INPUT_TEXT", self.input_text.clone());
        hashmap.insert("LOCALE", self.locale.clone());
        hashmap.insert("VOICE", self.voice.clone());
        hashmap.insert("INPUT_TYPE", "TEXT".to_string());
        hashmap.insert("OUTPUT_TYPE", "AUDIO".to_string());
        hashmap.insert("AUDIO", "WAVE".to_string());
        hashmap
    }
}

pub fn tts_blocking<W: std::io::Write>(srv: &TTSServer, query: &TTSQuery, writer: &mut W) -> Result<(), TTSError> {
    let client = reqwest::blocking::Client::new();
    let mut res = client.post(&format!("{}/process", srv.get_url()))
                    .form(&query.make_hashmap())
                    .send()?;
    res.copy_to(writer)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
