use card::Card;
use serde_json::Value;
use std::collections::HashMap;

mod card;

async fn sample_get() -> Result<(), Box<dyn std::error::Error>> {
    let v: Value =
        reqwest::get("https://shadowverse-portal.com/api/v1/cards?format=json&lang=ja&clan=1")
            .await?
            .json()
            .await?;

    let cards = v["data"]["cards"].as_array().unwrap();
    for x in cards {
        let obj: Card = serde_json::from_value(x.clone()).unwrap();
        println!("{:#?}", obj);
        println!("END");
    }
    //    let card = &v["data"]["cards"][0];
    //    serde_json::from_value(v["data"]["cards"][0]);
    //    println!("card: {:?}", card);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let resp = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(sample_get());
        resp.unwrap();
    }
}
