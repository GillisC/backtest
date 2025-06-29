
// bring in modules
pub mod engine;
pub mod strategy;
pub mod indicator;
pub mod plotter;

// expose parts of modules
pub use engine::{Order, Candle, OrderType, Trader, Report, ClosedTrade};
pub use indicator::{SMA};
pub use strategy::{SMACrossover};
pub use plotter::candle_plotter;



pub fn standard_deviation(data: &Vec<f64>) -> f64 {
    let len = data.len();
    if len == 0 {
        return 0.0;
    }

    let mean = data.iter().sum::<f64>() / len as f64;
    let variance = data.iter()
        .map(|x| (x - mean).powi(2))
        .sum::<f64>() / len as f64;

    variance.sqrt()
}

