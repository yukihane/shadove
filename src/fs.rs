use crate::card::Card;
use std::fs::File;
use std::io::{BufReader, BufWriter};

pub fn write(cards: &[Card]) {
    let writer = BufWriter::new(File::create("data.bin").unwrap());

    serde_json::to_writer_pretty(writer, cards).unwrap();
}

pub fn read() -> Vec<Card> {
    let reader = BufReader::new(File::open("data.bin").unwrap());
    serde_json::from_reader(reader).unwrap()
}
