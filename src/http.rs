use crate::card::Card;
use serde_json::Value;

pub async fn get() -> Result<Vec<Card>, Box<dyn std::error::Error>> {
    let v: Value =
        reqwest::get("https://shadowverse-portal.com/api/v1/cards?format=json&lang=ja&clan=1")
            .await?
            .json()
            .await?;

    let cards = v["data"]["cards"].as_array().unwrap().clone();
    let cards = convert(cards);

    Ok(cards)
}

fn convert(src: Vec<Value>) -> Vec<Card> {
    src.into_iter()
        //        .map(|x| {
        //            let text = format!("{:#?}", &x);
        //            let y = serde_json::from_value::<Card>(x);
        //            y.expect(&text)
        //        })
        .map(|x| serde_json::from_value::<Card>(x).unwrap())
        .collect::<Vec<Card>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let resp = tokio::runtime::Runtime::new().unwrap().block_on(get());
        let cards = resp.unwrap();
        println!("{:#?}", cards[3])
    }
}
