use shadove::fs::write;
use shadove::http::get;

fn main() {
    let resp = tokio::runtime::Runtime::new().unwrap().block_on(get());
    let cards = resp.unwrap();
    write(cards.as_slice());
}
