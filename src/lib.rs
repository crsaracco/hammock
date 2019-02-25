extern crate ratelimit;
extern crate reqwest;
extern crate serde_json;

use std::time::Duration;
use ratelimit::Limiter;

pub struct Hammock {
    reqwest_client: reqwest::Client,
    ratelimit: Limiter,
}

impl Hammock {
    /// Create a Hammock that rate-limits requests to one request per `duration`.
    pub fn new(duration: Duration, user_agent: &str) -> Result<Self, Box<std::error::Error>> {
        let ratelimit = ratelimit::Builder::new()
            .capacity(1) //number of tokens the bucket will hold
            .quantum(1) //add one token per interval
            .interval(duration) //add quantum tokens every `duration`
            .build();

        let mut reqwest_headers = reqwest::header::HeaderMap::new();
        reqwest_headers.insert(
            reqwest::header::USER_AGENT,
            reqwest::header::HeaderValue::from_str(user_agent)?
        );

        let reqwest_client = reqwest::Client::builder()
            .default_headers(reqwest_headers)
            .build()?;

        Ok(Self {
            reqwest_client,
            ratelimit,
        })
    }

    pub fn get_text(&mut self, url: &str) -> Result<String, Box<std::error::Error>> {
        self.ratelimit.wait();
        let response_body = self.reqwest_client.get(url)
            .send()?
            .text()?;
        Ok(response_body)
    }

    pub fn get_json(&mut self, url: &str) -> Result<serde_json::Value, Box<std::error::Error>> {
        self.ratelimit.wait();
        let json: serde_json::Value = self.reqwest_client.get(url)
            .send()?
            .json()?;
        Ok(json)
    }
}