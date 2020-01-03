use crate::card::Card;
use serde_json::Value;
use std::fs;
use std::fs::File;
use std::io::{BufReader, BufWriter, Error};

static SAVE_FILE: &str = "data.bin";

pub fn fetch_force() {
    let json = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(read_value_from_remote())
        .unwrap();
    write_value_to_local(&json).unwrap();
}

async fn read_value_from_remote() -> Result<Value, Box<dyn std::error::Error>> {
    let v: Value = reqwest::get("https://shadowverse-portal.com/api/v1/cards?format=json&lang=ja")
        .await?
        .json()
        .await?;

    Ok(v)
}

fn write_value_to_local(json: &Value) -> Result<(), Error> {
    let file = File::create(SAVE_FILE)?;
    let writer = BufWriter::new(file);

    serde_json::to_writer_pretty(writer, json)?;

    Ok(())
}

pub fn find_all() -> Result<Vec<Card>, Error> {
    let file = File::open(SAVE_FILE)?;
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).map_err(Error::from)
}

#[cfg(test)]
#[allow(clippy::assertions_on_constants)]
mod tests {
    use super::*;
    use std::io::ErrorKind;

    #[test]
    fn find_all_if_not_exist() {
        let _ = fs::remove_file(SAVE_FILE);

        match find_all() {
            Err(e) => assert_eq!(e.kind(), ErrorKind::NotFound),
            _ => assert!(false, "ファイルがないので失敗するはず"),
        }
    }
}
