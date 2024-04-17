mod data_structures;
mod get_stonks;
use tokio;

#[tokio::main]
async fn main() {
    let client = get_stonks::StockGetter::new("dwl20p9Z_3RabF7i1Fp7QdY99G9daCe4");

    println!("{:?}", client);

    let x = client.get_ticker_details("AMD").await.unwrap();

    println!("{:?}", x);
}
