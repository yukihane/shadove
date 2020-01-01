pub mod card;
pub mod fs;
pub mod http;

use crate::fs::write;
use crate::http::get;

pub use crate::fs::read as load_from_local;

pub fn fetch_from_remote() {
    let resp = tokio::runtime::Runtime::new().unwrap().block_on(get());
    let cards = resp.unwrap();
    write(cards.as_slice());
}
