use std::error::Error;
use crate::strategy::Strategy;
use crate::{OrderType, Order, Candle};

pub struct Trader {
    strategy: Box<dyn Strategy>,
    portfolio: Portfolio,
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

        // Liquidate if there is still a position
        if self.strategy.is_in_position() {
            let order = Order::new(OrderType::Sell, last_close_price, 10);
            self.portfolio.update(&order);
        }

        println!("Final balance: {}", self.portfolio.balance);

        Ok(())
    }
}

struct Portfolio {
    balance: f64,
    shares_long: usize,
    avg_entry: f64,
}

impl Portfolio {
    fn new(cash: f64) -> Self {
        Self {
            balance: cash,
            shares_long: 0,
            avg_entry: 0.0,
        }
    }

    fn update(&mut self, order: &Order) {
        match &order.order_type {
            OrderType::Buy => {
                self.shares_long += order.amount;
                let cost = order.price * order.amount as f64;
                self.balance -= cost;
                self.avg_entry = ( self.avg_entry * self.shares_long as f64 + cost ) / self.shares_long as f64;
            }
            OrderType::Sell => {
                self.shares_long -= order.amount;
                let cash = order.price * order.amount as f64;
                self.balance += order.price * order.amount as f64;
                self.avg_entry = ( self.avg_entry * self.shares_long as f64 + cash) / self.shares_long as f64;
            }
        }
    }
}
