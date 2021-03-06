// Copyright 2018-2020, Wayfair GmbH
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

//! HTTP / REST client

use crate::errors::*;
use reqwest;
use std::fmt;

/// HTTP Client wrapper
#[derive(Clone)]
pub struct HttpC {
    client: reqwest::Client,
    url: String,
}

impl fmt::Debug for HttpC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.url)
    }
}

impl HttpC {
    /// Creates a new http wrapper
    pub fn new(url: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            url,
        }
    }

    /// GET request
    pub fn get(&self, path: &str) -> Result<reqwest::RequestBuilder> {
        let fqurl = format!("{}{}", self.url, path);
        let endpoint: reqwest::Url = reqwest::Url::parse(&fqurl)?;
        Ok(self.client.get(endpoint))
    }
    /// PATCH request
    pub fn patch(&self, path: &str) -> Result<reqwest::RequestBuilder> {
        let fqurl = format!("{}{}", self.url, path);
        let endpoint: reqwest::Url = reqwest::Url::parse(&fqurl)?;
        Ok(self.client.patch(endpoint))
    }

    /// DELETE request
    pub fn delete(&self, path: &str) -> Result<reqwest::RequestBuilder> {
        let fqurl = format!("{}{}", self.url, path);
        let endpoint: reqwest::Url = reqwest::Url::parse(&fqurl)?;
        Ok(self.client.delete(endpoint))
    }
    /// POST request
    pub fn post(&self, path: &str) -> Result<reqwest::RequestBuilder> {
        let fqurl = format!("{}{}", self.url, path);
        let endpoint: reqwest::Url = reqwest::Url::parse(&fqurl)?;
        Ok(self.client.post(endpoint))
    }

    /// PUT request
    pub fn put(&self, path: &str) -> Result<reqwest::RequestBuilder> {
        let fqurl = format!("{}{}", self.url, path);
        let endpoint: reqwest::Url = reqwest::Url::parse(&fqurl)?;
        Ok(self.client.put(endpoint))
    }

    // pub fn head(&self, path: String) -> reqwest::RequestBuilder {
    //     let endpoint = format!("{}{}", self.url, path);
    //     self.client.head(endpoint.into())
    // }
}

#[cfg(test)]
mod test {
    use super::*;

    const ECHO: &'static str = "https://postman-echo.com";

    // NOTE We use postman's echo service for convenience

    #[test]
    fn test_post_method() -> Result<()> {
        let rest_cli = HttpC::new(ECHO.to_string());
        let res = dbg!(rest_cli.post("/post")?.send()?);
        assert_eq!(200, res.status());
        Ok(())
    }

    #[test]
    fn test_put_method() -> Result<()> {
        let rest_cli = HttpC::new(ECHO.to_string());
        let res = dbg!(rest_cli.put("/put")?.send()?);
        assert_eq!(200, res.status());
        Ok(())
    }
}
