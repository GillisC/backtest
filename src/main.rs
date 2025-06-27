use std::{io, process};
use std::error::Error;

use serde::{Deserialize, Serialize};
use csv;

#[derive(Debug, Deserialize)]
struct Candle {
    date: String,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
}

fn example() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.deserialize() {
        let record: Candle = result?;
        println!("{:?}", record);
    }

    Ok(())
}

fn main() {
    if let Err(err) = example() {
        println!("error running example: {err}");
        process::exit(1)
    }    
}
