use std::error::Error;
use crate::strategy::Strategy;
use crate::{OrderType, Order, Candle};
use crate::engine::{Portfolio};

pub struct Trader {
    strategy: Box<dyn Strategy>,
    pub portfolio: Portfolio,
}

impl Trader {
    pub fn new(strat: Box<dyn Strategy>, starting_balance: f64) -> Self {
        Self {
            strategy: strat,
            portfolio: Portfolio::new(starting_balance),
        }
    }
    // TODO: make the csv feed an argument which the trader uses
    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_path("bitcoin_1h_1000.csv")?;
        // let mut rdr = csv::Reader::from_path("bitcoin_1h_1000.csv")
        //     .expect("csv file not found");


        for ( index, result ) in rdr.deserialize().enumerate() {
            let record: Candle = result?;

            if let Some(order) = self.strategy.on_candle(&record) {
                self.portfolio.update(&order, index);

                // temp
                match order.order_type {
                    OrderType::Buy => {
                        println!(
                            "bought {}x at {}, current balance: {}", 
                            order.amount, order.price, self.portfolio.balance);
                    }
                    OrderType::Sell => {
                        println!(
                            "sold   {}x at {}, current balance: {}", 
                            order.amount, order.price, self.portfolio.balance);
                    }
                }
            }
        }
        println!("Final balance: {}", self.portfolio.balance);

        Ok(())
    }
}

