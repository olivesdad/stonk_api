mod get_stonks;
use tokio;
use serde::{Serialize, Deserialize};

#[tokio::main]
async fn main() {
    get_stonks::test_mod();

    let client = get_stonks::StockGetter::new("dwl20p9Z_3RabF7i1Fp7QdY99G9daCe4");

    println!("{:?}", client);

    let x = client.get_details("ticker").await.unwrap();

    
    println!("{:?}", x.results.description);
}
