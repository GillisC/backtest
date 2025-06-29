use serde::{Deserialize};
use chrono::{ NaiveDateTime, DateTime, Utc };

pub mod trader;
pub mod portfolio;
pub mod report;

pub use trader::Trader;
pub use portfolio::Portfolio;
pub use report::Report;

#[derive(Debug, Deserialize)]
pub struct Candle {
    pub date: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

#[derive(Clone, PartialEq, Eq)]
pub enum OrderType {
    Buy,
    Sell,
}

#[derive(Clone)]
pub struct Order {
    pub order_type: OrderType,
    pub price: f64,
    pub amount: usize,
}

impl Order {
    pub fn new(t: OrderType, price: f64, amount: usize) -> Self {
        Self {
            order_type: t,
            price: price,
            amount: amount,
        }
    }
}

pub struct ClosedTrade {
    pub realized_pnl: f64,
}

impl ClosedTrade {
    pub fn new(pnl: f64) -> Self {
        Self {
            realized_pnl: pnl,
        }
    }
}

