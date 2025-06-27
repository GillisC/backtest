use crate::{Candle, Order};

pub mod sma_crossover;

pub use sma_crossover::SMACrossover; 

pub trait Strategy {
    fn on_candle(&mut self, candle: &Candle) -> Option<Order>;
    fn is_in_position(&self) -> bool;
}
