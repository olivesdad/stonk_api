use std::iter::Map;

use reqwest::{header, Client, Error, RequestBuilder, Response};
use serde::{Serialize, Deserialize};

pub fn test_mod() {
    println!("hello world!")
}

#[derive(Deserialize)]
pub struct Payload {
   pub results: Results
}
#[derive(Deserialize)]
pub struct Results{
    pub description: String,
    pub ticker: String,
}

#[derive(Debug)]
pub struct StockGetter {
    requester: Client,
}

impl StockGetter {
    pub fn new<'a>(key: &'a str) -> Self {

        let auth = ["Bearer ", key].concat(); 
        let mut headers = header::HeaderMap::new();
        headers.insert("Authorization", header::HeaderValue::from_str(&auth).unwrap());
        Self {
            requester: reqwest::Client::builder()
                .default_headers(headers)
                .build()
                .unwrap(),
        }
    }

    pub async fn get_details(&self, ticker: &str) -> Result<Payload, Error> {
        self.requester.get("https://api.polygon.io/v3/reference/tickers/AAPL").send().await.unwrap()
        .json::<Payload>()
        .await
    }
}
