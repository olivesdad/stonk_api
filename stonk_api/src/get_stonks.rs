use std::iter::Map;

use reqwest::{header, Client, Error, RequestBuilder, Response};
use serde::{Serialize, Deserialize};

pub fn test_mod() {
    println!("hello world!")
}

#[derive(Deserialize, Debug)]
pub struct Payload {
   pub results: Results
}
#[derive(Deserialize, Debug)]
pub struct Results{
    pub description: Option<String>,
    pub ticker: Option<String>,
    pub active: Option<bool>,
    pub currency_name: Option<String>,
    pub primary_exchange: Option<String>,


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
