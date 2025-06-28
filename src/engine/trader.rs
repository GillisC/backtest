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

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let mut rdr = csv::Reader::from_path("bitcoin_1h_1000.csv")
            .expect("csv file not found");

        let mut last_close_price: f64 = 0.0;

        for result in rdr.deserialize() {
            let record: Candle = result?;
            last_close_price = record.close;

            if let Some(order) = self.strategy.on_candle(&record) {
                self.portfolio.update(&order);
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

