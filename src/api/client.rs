use std::time::Duration;

use reqwest::cookie::CookieStore;
use reqwest::header::{HeaderMap, HeaderValue, REFERER};
use reqwest::{cookie::Cookie, Client, Result};

const BASE_URL: &str = "https://rentry.co";

#[derive(Debug)]
pub struct HttpAdapter {
    client: Client,
}

impl HttpAdapter {
    pub fn new() -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(15))
            .default_headers(Self::headers())
            .cookie_store(true)
            .build()?;

        Ok(Self { client })
    }

    pub async fn csrf_token<'a>(&self) -> Result<String> {
        let request = self.client.get(BASE_URL).send().await?;
        let response: Vec<Cookie> = request
            .cookies()
            .filter(|cookie| cookie.name().eq("csrftoken"))
            .collect();
        println!("tokens: {:?}", response);
        Ok(response[0].value().to_string())
    }

    fn headers() -> HeaderMap {
        let mut headers = HeaderMap::new();
        let referer_value = HeaderValue::from_static(BASE_URL);
        headers.insert(REFERER, referer_value);
        headers
    }
}
