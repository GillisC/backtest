use backtest::{Trader};
use backtest::strategy::SMACrossover;

fn main() {
    let strat = Box::new( SMACrossover::new() );
    let mut trader = Trader::new(strat, 10_000_000.0);
    if let Err(e) = trader.run() {
        println!("Error occured: {e}");
    }
}
