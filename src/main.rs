use backtest::{Trader, Report};
use backtest::strategy::SMACrossover;


fn main() {
    let strat = Box::new( SMACrossover::new() );
    let mut trader = Trader::new(strat, 10_000_000.0);
    if let Err(e) = trader.run() {
        println!("Error occured: {e}");
   }
    println!("realized profit/loss: {}", trader.portfolio.realized_pnl);
    let report = Report::build(&trader.portfolio);
    println!("Final report: {report}");
}
