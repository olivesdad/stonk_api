use crate::data_structures;
use reqwest::{header, Client, Error};
use chrono::{DateTime, Utc};


#[derive(Debug)]
pub struct StockGetter {
    requester: Client,
}

pub enum EndPoint {

    TickerDetails: String,
    

}
impl StockGetter {
    pub fn new<'a>(key: &'a str) -> Self {
        let auth = ["Bearer ", key].concat();
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Authorization",
            header::HeaderValue::from_str(&auth).unwrap(),
        );

        // use request builder to make client
        Self {
            requester: reqwest::Client::builder()
                .default_headers(headers)
                .build()
                .unwrap(), // Just unwrap it because if we fail to build the client we're dead
        }
    }

    // Send request to tickers endpoint to get info about the ticker passed to the fn
    pub async fn get_ticker_details(&self, tick: &str) -> Result<data_structures::Ticker, Error> {
        // form url
        let url: String = ["https://api.polygon.io/v3/reference/tickers/", tick.to_uppercase().as_ref()].concat();

        // Send request and await response
        self.requester
            .get(url)
            .send()
            .await?
            .json::<data_structures::Ticker>()
            .await
    }

    pub async fn get_daily(&self, tick: &str, date: DateTime<Utc>) -> Result<data_structures::Daily, ()> {

        Err(())
    }
}
