use crate::card::Card;
use std::fs::File;
use std::io::BufWriter;

pub fn write(cards: &[Card]) {
    let writer = BufWriter::new(File::create("data.bin").unwrap());

    serde_json::to_writer_pretty(writer, cards).unwrap();
}
